mod tasks;
mod scheduler;
pub(crate) mod task_list;

use mlua::prelude::*;

#[mlua::lua_module]
fn scheduler_coroutines(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    exports.set("fifo", lua.create_function(crate::scheduler::fifo)?)?;
    exports.set("lottery", lua.create_function(crate::scheduler::lottery)?)?;

    Ok(exports)
}