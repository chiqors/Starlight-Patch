use ilhook::x64::{CallbackOption, HookFlags, HookPoint, HookType, Hooker, JmpBackRoutine};

pub struct HookMgr {
    pub points: Vec<HookPoint>,
}

impl HookMgr {
    pub fn new() -> HookMgr {
        Self {
            points: Vec::new(),
        }
    }

    pub unsafe fn hook(&mut self, addr: usize, routine: JmpBackRoutine) {
        let hooker = Hooker::new(
            addr,
            HookType::JmpBack(routine),
            CallbackOption::None,
            0,
            HookFlags::empty()
        );

        self.points.push(
            hooker.hook().unwrap()
        );
    }
}