use super::TaskList;
use crate::tasks::Task;
use rand::{prelude::*, rng};

pub(crate) struct Lottery {
    tasks: Vec<Task>,
    randomizer: ThreadRng,
}

impl TaskList for Lottery {
    type That = Self;

    fn new() -> Self::That {
        Self {
            tasks: Vec::new(),
            randomizer: rng(),
        }
    }

    fn peek(&mut self) -> Option<Task> {
        if self.is_empty() {
            return None;
        }
        let index = self.choose();
        Some(self.tasks.remove(index))
    }

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Lottery {
    fn choose(&mut self) -> usize {
        let ticket = self.randomizer.random_range(0..self.total_tickets());
        self.index_of_ticket(ticket)
    }

    fn total_tickets(&self) -> usize {
        self.tasks.iter().map(|task| task.priority() as usize).sum()
    }

    fn index_of_ticket(&self, ticket: usize) -> usize {
        let mut total_tickets = 0;

        for (idx, task) in self.tasks.iter().enumerate() {
            total_tickets += task.priority() as usize;

            if total_tickets >= ticket {
                return idx;
            }
        }

        0
    }
}
