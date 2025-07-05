mod fifo;
mod lottery;

use crate::tasks::Task;

pub(crate) use fifo::FIFOTaskList;
pub(crate) use lottery::Lottery;

pub(crate) trait TaskList {

    type That;

    fn new() -> Self::That;

    fn peek(&mut self) -> Option<Task>;

    fn add(&mut self, task: Task);

    fn is_empty(&self) -> bool;

}