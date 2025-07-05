use mlua::prelude::*;

use crate::{task_list::TaskList, tasks::Task};

pub(crate) struct LuaScheduler<Tasks: TaskList + 'static> {
    tasks: Tasks,
    life_time: usize
}

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {

    pub(super) fn new(tasks: Tasks) -> Self {
        Self {
            tasks,
            life_time: 0
        }
    }

    pub(super) fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }

    pub(super) fn run(&mut self) {
        while self.has_tasks() {
            self.step();
        }
    }

    pub(super) fn steps(&mut self, count: LuaInteger) {
        (0..count).for_each(|_| self.step());
    }

    pub(super) fn add_task(&mut self, coroutine: LuaThread, priority: LuaInteger) {
        self.tasks.add(Task::new(coroutine, priority));
    }
}

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {
    fn step(&mut self) {
        let mut task = match self.tasks.peek() {
            Some(t) => t,
            None => return
        };

        task.resume();
        self.life_time += 1;

        if task.is_alive() {
            self.tasks.add(task);
        }
    }
}