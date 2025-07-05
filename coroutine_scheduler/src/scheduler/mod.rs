mod lua;
mod rust;

use mlua::prelude::*;
use rust::LuaScheduler;
use crate::task_list::*;

pub(crate) fn fifo(_: &Lua, _: ()) -> LuaResult<LuaScheduler<FIFOTaskList>> {
    Ok(LuaScheduler::new(FIFOTaskList::new()))
}

pub(crate) fn lottery(_: &Lua, _:()) -> LuaResult<LuaScheduler<Lottery>> {
    Ok(LuaScheduler::new(Lottery::new()))
}