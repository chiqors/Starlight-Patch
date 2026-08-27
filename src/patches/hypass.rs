use std::sync::OnceLock;
use ilhook::x64::Registers;
use windows::core::s;
use windows::Win32::Networking::WinHttp::WINHTTP_FLAG_SECURE;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use crate::config::CONFIG;
use crate::patches::{Fireable, Patch};

static HOST: OnceLock<Vec<u16>> = OnceLock::new();

pub struct HYPass;

impl Fireable for Patch<HYPass> {
    unsafe fn fire(&mut self) -> Result<(), ()> {
        let winhttp = GetModuleHandleA(s!("winhttp.dll")).map_err(|_| ())?;
        let connect = GetProcAddress(winhttp, s!("WinHttpConnect")).unwrap() as usize;
        let openrequest = GetProcAddress(winhttp, s!("WinHttpOpenRequest")).unwrap() as usize;

        self.hook_mgr.hook(connect, on_connect);
        self.hook_mgr.hook(openrequest, on_open_request);
        Ok(())
    }
}

unsafe extern "win64" fn on_connect(reg: *mut Registers, _: usize) {
    let config = CONFIG.get().unwrap().read().unwrap();

    let host = HOST.get_or_init(|| {
        config.network.address
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect()
    });

    (*reg).rdx = host.as_ptr() as u64;
    (*reg).r8 = config.network.port as u64;
}

unsafe extern "win64" fn on_open_request(reg: *mut Registers, _: usize) {
    let flags_ptr = ((*reg).rsp + 0x38) as *mut u32;
    *flags_ptr &= !WINHTTP_FLAG_SECURE.0;
}