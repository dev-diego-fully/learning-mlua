mod event_handler;
mod lua_event_handler;
mod event_set;

use crate::lua_event_handler::LuaEventHandler;
use mlua::prelude::*;

#[mlua::lua_module]
fn lua_events(lua: &Lua) -> LuaResult<LuaEventHandler> {
    LuaEventHandler::lua_new(lua, ())
}
