use crate::event_handler::EventHandler;
use mlua::prelude::*;

pub(crate) struct LuaEventHandler {
    inner_handler: EventHandler,
}

impl LuaEventHandler {
    pub(crate) fn lua_new(_: &Lua, (): ()) -> LuaResult<Self> {
        Ok(Self {
            inner_handler: EventHandler::new(),
        })
    }

    fn lua_add_event_listener(
        _: &Lua,
        this: &mut Self,
        (event_name, listeners): (LuaString, LuaVariadic<LuaFunction>),
    ) -> LuaResult<()> {
        this.inner_handler.when(&event_name, &listeners);
        Ok(())
    }

    fn lua_trigger_event(
        lua: &Lua,
        this: &mut Self,
        (event_name, datas): (LuaString, LuaTable),
    ) -> LuaResult<()> {
        this.inner_handler.trigger_event(lua, &event_name, &datas)
    }
}


impl LuaUserData for LuaEventHandler {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", Self::lua_new);
        methods.add_method_mut("when", Self::lua_add_event_listener);
        methods.add_method_mut("trigger", Self::lua_trigger_event);
    }
}
