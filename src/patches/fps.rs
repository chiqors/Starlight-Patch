use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use ilhook::x64::Registers;
use crate::config::CONFIG;
use crate::patches::{Fireable, Patch};
use crate::{scanner, GAME_BASE};

pub static SET_TARGET_FPS: &str = "E8 ? ? ? ? E8 ? ? ? ? 83 F8 1F 0F 9C 05";
pub static WORKS: AtomicBool = AtomicBool::new(false);

thread_local! {
    pub static DISABLE_HOOK: Cell<bool> = Cell::new(false);
}

pub struct Fps;

pub fn update_frame_rate(should_reset: bool) {
    unsafe {
        static PTR_LOCK: OnceLock<usize> = OnceLock::new();

        let ptr = PTR_LOCK.get_or_init(|| {
            let ptr = scanner::scan_ga_section(SET_TARGET_FPS)
                .expect("Can't update frame rate: patch outdated");

            if ptr != 0 {
                ptr + *GAME_BASE
            } else {
                ptr
            }
        });

        type SetTargetFpsFn = unsafe extern "win64" fn(fps: i32);
        let func: SetTargetFpsFn = std::mem::transmute(*ptr);

        if should_reset {
            DISABLE_HOOK.set(true);
            func(60);
            DISABLE_HOOK.set(false);
        } else {
            // shouldn't matter
            func(123);
        }
    }
}

unsafe extern "win64" fn on_framerate_change(reg: *mut Registers, _: usize) {
    let config = CONFIG.get().unwrap().read()
        .expect("Config lock is poisoned");

    if config.fps.enabled && !DISABLE_HOOK.get() {
        (*reg).rcx = config.fps.target_max_fps as u64;
    }
}

impl Fireable for Patch<Fps> {
    unsafe fn fire(&mut self) -> Result<(), ()> {
        let set_target_fps = scanner::scan_ga_section(SET_TARGET_FPS)
            .unwrap_or(0);

        if set_target_fps == 0 {
            tracing::warn!("SET_TARGET_FPS failed! Does the patch need to be updated?")
        } else {
            WORKS.store(true, Ordering::Relaxed)
        }

        self.hook_mgr.hook(
            set_target_fps + *GAME_BASE,
            on_framerate_change
        );

        Ok(())
    }
}