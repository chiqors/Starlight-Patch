use ilhook::x64::Registers;
use crate::config::CONFIG;
use crate::patches::{Fireable, Patch};
use crate::{misc, scanner, GAME_BASE};

const WEB_REQUEST_UTILS_MAKE_INITIAL_URL: &str = "55 41 56 56 57 53 48 81 EC ? ? ? ? 48 8D AC 24 ? ? ? ? 48 C7 45 ? ? ? ? ? 48 89 D6 48 89 CF 48 8B 0D ? ? ? ?";
const BROWSER_LOAD_URL: &str = "41 B0 01 E9 08 00 00 00 0F 1F 84 00 00 00 00 00 56 57";
const BROWSER_LOAD_URL_OFFSET: usize = 0x10;

pub struct Redirect;

impl Fireable for Patch<Redirect> {
    unsafe fn fire(&mut self) -> Result<(), ()> {
        let web_request_utils_make_initial_url = scanner::scan_ga_section(WEB_REQUEST_UTILS_MAKE_INITIAL_URL)
            .expect("WEB_REQUEST_UTILS_MAKE_INITIAL_URL failed! Does the patch need to be updated?")
            + *GAME_BASE;

        let browser_load_url = scanner::scan_ga_section(BROWSER_LOAD_URL)
            .expect("BROWSER_LOAD_URL failed! Does the patch need to be updated?")
            + *GAME_BASE
            + BROWSER_LOAD_URL_OFFSET;

        self.hook_mgr.hook(web_request_utils_make_initial_url, on_make_initial_url);
        self.hook_mgr.hook(browser_load_url, on_browser_load_url);

        tracing::info!("Started redirecting!");
        Ok(())
    }
}

unsafe extern "win64" fn on_make_initial_url(reg: *mut Registers, _: usize) {
    let str_length = *((*reg).rcx.wrapping_add(16) as *const u32);
    let str_ptr = (*reg).rcx.wrapping_add(20) as *const u8;

    let slice = std::slice::from_raw_parts(str_ptr, (str_length * 2) as usize);
    let old_url = String::from_utf16le(slice).unwrap();

    let config = CONFIG.get().unwrap();
    let use_https = config.network.https;
    let mut url = format!(
        "http{}://{}:{}",
        if use_https == true { "s" } else { "" },
        config.network.address,
        config.network.port
    );

    old_url.split('/').skip(3).for_each(|s| {
        url.push('/');
        url.push_str(s);
    });

    if !old_url.contains("/query_cur_region") && !old_url.starts_with("file://") {
        tracing::debug!("{} -> {}", old_url, url);
        (*reg).rcx = misc::create_il2cpp_string(url.as_str()) as u64;
    }
}

unsafe extern "win64" fn on_browser_load_url(reg: *mut Registers, _: usize) {
    let str_length = *((*reg).rdx.wrapping_add(16) as *const u32);
    let str_ptr = (*reg).rdx.wrapping_add(20) as *const u8;

    let slice = std::slice::from_raw_parts(str_ptr, (str_length * 2) as usize);
    let old_url = String::from_utf16le(slice).unwrap();

    let config = CONFIG.get().unwrap();
    let use_https = config.network.https;
    let mut new_url = format!(
        "http{}://{}:{}",
        if use_https == true { "s" } else { "" },
        config.network.address,
        config.network.port
    );

    old_url.split('/').skip(3).for_each(|s| {
        new_url.push('/');
        new_url.push_str(s);
    });

    (*reg).rdx = misc::create_il2cpp_string(new_url.as_str()) as u64;
}