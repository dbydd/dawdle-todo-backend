mod basic_priorty_queue;
mod once;
use std::{panic::set_hook, rc::Rc, sync::Arc};

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

use crate::configurations::Configurations;

use self::once::OnceContainer;

use super::{
    modifiers,
    task::{InternalDate, Priorty, Task},
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
    fn priorty(&self, center: &TaskDataCenter) -> Priorty;
    fn times_remain(&self, center: &TaskDataCenter) -> TimeDelta;
}

#[derive(Serialize, Deserialize)]
struct DefaultContainer {}
