use std::marker::PhantomData;
use crate::hookmgr::HookMgr;

pub mod redirect;
pub mod encryption;
pub mod hypass;

#[derive(Default)]
pub struct PatchManager {
    patches: Vec<Box<dyn Fireable>>
}

impl PatchManager {
    pub unsafe fn run(&mut self, patch: impl Fireable + 'static) {
        let mut boxed = Box::new(patch);
        
        boxed.fire().unwrap();
        self.patches.push(boxed);
    }
}

unsafe impl Sync for PatchManager {}
unsafe impl Send for PatchManager {}

pub struct Patch<T> {
    pub hook_mgr: HookMgr,
    pub _marker: PhantomData<T>,
}

pub trait Fireable {
    unsafe fn fire(&mut self) -> Result<(), ()>;
}

impl<T> Patch<T> {
    pub fn new() -> Patch<T> {
        Self {
            hook_mgr: HookMgr::new(),
            _marker: PhantomData,
        }
    }
}