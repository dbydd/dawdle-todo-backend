mod basic_priority_queue;
mod once;
#[cfg(test)]
mod test_container;

use std::sync::Arc;

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    task::{Priority, Task},
    TaskDataCenter,
};

// pub enum EnumTaskContainer {
//     EnumTypeOnceContainer(OnceContainer),
//     EnumTypeDefaultContainer(DefaultContainer),
// }

pub(crate) trait TaskContainer {
    fn id(&self) -> &str;
    fn peek_task_inner(&self, center: &TaskDataCenter) -> Arc<Task>; //考虑实现为iter?
    fn complete_current_task_once(&mut self, center: &mut TaskDataCenter);
    fn fully_completed(&self, center: &TaskDataCenter) -> bool;
    fn priority(&self, center: &TaskDataCenter) -> Priority;
    fn times_remain(&self, center: &TaskDataCenter) -> TimeDelta;
    fn to_json(&self, center: &TaskDataCenter) -> Option<String>;
}

impl TaskContainer {
    pub(crate) fn get_container_info(&self, center: &TaskDataCenter) -> Value {
        // json!(
        //     {
        //     id:self.id(),
        //     task_inner:self.peek_task_inner(center)
        // }
        // )
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
struct DefaultContainer {}
