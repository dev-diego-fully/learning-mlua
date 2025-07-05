use mlua::prelude::*;

pub(crate) struct Task {
    coroutine: LuaThread,
    priority: LuaInteger
}

impl Task {
    pub(crate) fn new(coroutine: LuaThread, priority: LuaInteger) -> Self {
        Self {
            coroutine,
            priority
        }
    }

    pub(crate) fn priority(&self) -> LuaInteger {
        self.priority
    }

    fn step(&mut self) {
        let _ = self.coroutine.resume::<()>(());
    }

    pub(crate) fn is_alive(&self) -> bool {
        match self.coroutine.status() {
            LuaThreadStatus::Resumable | LuaThreadStatus::Running => true,
            LuaThreadStatus::Finished | LuaThreadStatus::Error => false,
        }
    }

    pub(crate) fn resume(&mut self) {
        if matches!(self.coroutine.status(), LuaThreadStatus::Resumable) {
            self.step();
        }
    }
}
