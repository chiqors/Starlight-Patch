#![allow(unsafe_op_in_unsafe_fn)]

mod misc;
mod patches;
mod hookmgr;
mod config;
mod protection;
mod scanner;
mod ui;

use std::ffi::c_void;
use std::sync::RwLock;
use hudhook::hooks::dx11::ImguiDx11Hooks;
use lazy_static::lazy_static;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use crate::misc::is_wine;
use crate::patches::{Patch, PatchManager};
use crate::patches::encryption::Encryption;
use crate::patches::fps::Fps;
use crate::patches::hypass::HYPass;
use crate::patches::redirect::Redirect;

#[cfg(debug_assertions)]
const LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: tracing::Level = tracing::Level::INFO;

lazy_static! {
    pub static ref GAME_BASE: usize = unsafe { GetModuleHandleW(None).unwrap().0 as usize };
    static ref PATCH_MANAGER: RwLock<PatchManager> = RwLock::new(PatchManager::default());
}

unsafe fn main() {
    std::thread::sleep(std::time::Duration::from_secs(2));

    unsafe { AllocConsole().unwrap() };

    tracing_subscriber::fmt()
        .with_ansi(!is_wine())
        .with_max_level(LOG_LEVEL)
        .with_target(false)
        .init();

    ansi_term::enable_ansi_support().unwrap();

    misc::print_banner();
    protection::disable();

    tracing::info!("Starlight RSA patch V{}", env!("CARGO_PKG_VERSION"));
    config::loader::load_config();

    PATCH_MANAGER.write().unwrap().run(Patch::<Redirect>::new());
    PATCH_MANAGER.write().unwrap().run(Patch::<Encryption>::new());
    PATCH_MANAGER.write().unwrap().run(Patch::<HYPass>::new());
    PATCH_MANAGER.write().unwrap().run(Patch::<Fps>::new());
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    _: *mut c_void,
) {
    if reason == DLL_PROCESS_ATTACH {
        let hmodule = hmodule.0 as usize;

        std::thread::spawn(move || main());
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let hmodule = HINSTANCE(hmodule as _);
            hudhook::Hudhook::builder()
                .with::<ImguiDx11Hooks>(ui::RenderLoop { ui_visible: true, toggle_pressed: false })
                .with_hmodule(hmodule)
                .build()
                .apply()
                .unwrap();
        });
    }
}