use std::{thread, time::Duration};

use mlua::prelude::*;

fn say(_: &mlua::Lua, message: mlua::String) -> LuaResult<()> {
    println!("[rust]: {}", message.to_str()?);
    Ok(())
}

fn sleep(_: &Lua, millis: mlua::Number) -> LuaResult<()> {
    let time = Duration::from_millis(millis as u64);
    thread::sleep(time);
    Ok(())
}

#[mlua::lua_module]
fn hello_world(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("say", lua.create_function(say)?)?;

    exports.set("sleep", lua.create_function(sleep)?)?;

    Ok(exports)
}
