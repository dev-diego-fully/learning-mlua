use mlua::prelude::*;

use super::rust::LuaScheduler;
use crate::{task_list::TaskList};

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {
    fn lua_steps(_: &Lua, this: &mut Self, steps: Option<LuaInteger>) -> LuaResult<()> {
        let count = steps.unwrap_or(1);

        if count <= 0 {
            return Err(LuaError::runtime("Cant execute non positive steps count"));
        }

        this.steps(count);
        Ok(())
    }

    fn lua_has_tasks(_: &Lua, this: &Self, _: ()) -> LuaResult<LuaValue> {
        Ok(LuaValue::Boolean(this.has_tasks()))
    }

    fn lua_run(_: &Lua, this: &mut Self, _: ()) -> LuaResult<()> {
        this.run();
        Ok(())
    }

    fn lua_spawn_task(
        lua: &Lua,
        this: &mut Self,
        (function, priority): (LuaFunction, Option<LuaInteger>),
    ) -> LuaResult<()> {
        let prior = priority.unwrap_or(1);

        if prior <= 0 {
            return Err(LuaError::runtime("Can't deal with non positive priority"));
        }

        let coroutine = lua.create_thread(function)?;

        this.add_task(coroutine, prior);
        Ok(())
    }
}

impl<Tasks: TaskList + 'static> LuaUserData for LuaScheduler<Tasks> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("has_tasks", Self::lua_has_tasks);
        methods.add_method_mut("step", Self::lua_steps);
        methods.add_method_mut("run", Self::lua_run);
        methods.add_method_mut("spawn_task", Self::lua_spawn_task);
    }
}
