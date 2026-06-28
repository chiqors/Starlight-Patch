#![feature(str_from_utf16_endian)]
#![allow(unsafe_op_in_unsafe_fn)]

mod misc;
mod patches;
mod hookmgr;
mod config;
mod protection;
mod scanner;

use std::ffi::c_void;
use std::sync::RwLock;
use lazy_static::lazy_static;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use crate::misc::is_wine;
use crate::patches::{Patch, PatchManager};
use crate::patches::encryption::Encryption;
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
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(_: HINSTANCE, reason: u32, _: *mut c_void) {
    if reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || main());
    }
}