use std::collections::VecDeque;

use crate::tasks::Task;

use super::TaskList;

pub(crate) struct FIFOTaskList {
    tasks: VecDeque<Task>
}

impl TaskList for FIFOTaskList {

    type That = Self;

    fn new() -> Self::That {
        Self {
            tasks: VecDeque::new()
        }
    }
    
    fn peek(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    fn add(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}