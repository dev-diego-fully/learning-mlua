use std::collections::{HashMap, VecDeque};

use mlua::prelude::*;

use crate::event_set::ListenersSet;

pub(crate) struct EventHandler {
    events_and_listeners: HashMap<LuaString, ListenersSet>,
    listen_queue: VecDeque<(LuaFunction, LuaTable)>,
    treating_calls: bool,
}

impl EventHandler {
    pub(crate) fn new() -> Self {
        Self {
            events_and_listeners: HashMap::new(),
            listen_queue: VecDeque::new(),
            treating_calls: false,
        }
    }

    pub(crate) fn when(&mut self, event_name: &LuaString, happens: &LuaVariadic<LuaFunction>) {
        happens
            .iter()
            .for_each(|listener| self.add_listener(event_name, listener));
    }

    pub(crate) fn trigger_event(
        &mut self,
        lua: &Lua,
        event_name: &LuaString,
        datas: &LuaTable,
    ) -> LuaResult<()> {
        self.schedule_event_call(lua, event_name, datas)?;

        if !self.treating_calls {
            self.treat_calls()?;
        }

        Ok(())
    }

    pub(crate) fn remove_event_listener(
        &mut self,
        event_name: &LuaString,
        listeners: LuaVariadic<LuaFunction>,
    ) {
        if let Some(inners) = self.events_and_listeners.get_mut(event_name) {
            listeners.iter().for_each(|listener|inners.remove(listener));
        }
    }
}

impl EventHandler {
    fn add_listener(&mut self, event_name: &LuaString, listener: &LuaFunction) {
        match self.events_and_listeners.get_mut(event_name) {
            Some(listeners) => listeners.insert(listener),
            None => {
                self.events_and_listeners
                    .insert(event_name.clone(), ListenersSet::once(listener));
            }
        }
    }

    fn schedule_event_call(
        &mut self,
        lua: &Lua,
        event_name: &LuaString,
        datas: &LuaTable,
    ) -> LuaResult<()> {
        match self.events_and_listeners.get_mut(event_name) {
            Some(listeners) => {
                for l in listeners.listeners() {
                    self.listen_queue.push_back((l.clone(), event(lua, datas)?));
                }
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn treat_calls(&mut self) -> LuaResult<()> {
        self.treating_calls = true;

        while !self.listen_queue.is_empty() {
            let (lister, data) = self.listen_queue.pop_front().unwrap();
            lister.call::<()>(LuaValue::Table(data))?;
        }

        self.treating_calls = false;

        Ok(())
    }
}

fn event(lua: &Lua, datas: &LuaTable) -> LuaResult<LuaTable> {
    let event_table = lua.create_table()?;

    event_table.set("values", datas)?;

    Ok(event_table)
}