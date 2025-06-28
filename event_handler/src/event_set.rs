use mlua::Function as LuaFunction;
use std::collections::{HashSet};

pub(crate) struct ListenersSet {
    inner_set: HashSet<Listener>,
}

impl ListenersSet {
    pub(crate) fn once(first: &LuaFunction) -> Self {
        let mut this = Self {
            inner_set: HashSet::new(),
        };
        this.insert(first);
        this
    }

    pub(crate) fn insert(&mut self, element: &LuaFunction) {
        self.inner_set.insert(Listener::from(element));
    }

    pub(crate) fn remove(&mut self, element: &LuaFunction) {
        self.inner_set.remove(&Listener::from(element));
    }

    pub(crate) fn listeners(&self) -> impl Iterator<Item = &LuaFunction> {
        self.inner_set.iter().map(|node| node.function())
    }
}

struct Listener(LuaFunction);

impl Listener {
    fn from(function: &LuaFunction) -> Self {
        Self(function.clone())
    }

    fn function(&self) -> &LuaFunction {
        &self.0
    }
}

impl std::hash::Hash for Listener {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_pointer().hash(state);
    }
}

impl PartialEq for Listener {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_pointer() == other.0.to_pointer()
    }
}

impl Eq for Listener {}