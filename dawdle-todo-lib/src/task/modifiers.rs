use chrono::{DateTime, Utc};

use super::{Priorty, Task, TaskContainer};

pub fn simple_in_time_complete(taskinner: &dyn TaskContainer) -> Priorty {
    let task_inner = taskinner.peek_task_inner();
    let times_remain = taskinner.times_remain();
    todo!(); //TODO 写个日期比较器
}
