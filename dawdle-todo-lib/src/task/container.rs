use std::{panic::set_hook, rc::Rc, sync::Arc};

use serde::{Deserialize, Serialize};

use super::{
    dataCenter::{CONTAINER_LIST, TASKLIST},
    modifiers, Priorty, Task, TaskContainer,
};

#[derive(Serialize, Deserialize)]
struct OnceContainer {
    task: String,
    current_time: usize,
}

#[derive(Serialize, Deserialize)]
struct DefaultContainer {}

impl TaskContainer for OnceContainer {
    fn peek_task_inner(&self) -> Arc<Task> {
        TASKLIST.get().unwrap().get(&self.task).unwrap().clone()
    }

    fn complete_current_task_once(&mut self) {
        self.current_time += 1;
    }

    fn priorty(&self) -> Priorty {
        modifiers::simple_in_time_complete(self)
    }

    fn times_remain(&self) -> usize {
        todo!()
    }

    fn id(&self) -> &str {
        &self.task
    }

    fn fully_completed(&self) -> bool {
        self.current_time >= self.peek_task_inner().complete_time
    }
}
