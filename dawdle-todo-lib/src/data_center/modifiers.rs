use std::ops::Sub;

extern crate toml;

use crate::data_center::task::InternalDate;

use super::{
    container::TaskContainer,
    task::{Priorty, Task},
    TaskDataCenter,
};

pub fn simple_in_time_complete(
    taskinner: &dyn TaskContainer,
    center: &TaskDataCenter,
) -> Option<Priorty> {
    let task_inner = taskinner.peek_task_inner(center);
    let times_remain = taskinner.times_remain(center);
    let total_range =
        InternalDate(task_inner.end_date.clone()) - InternalDate(task_inner.begin_date.clone());
    match Some(times_remain.num_hours() / total_range.num_hours()) {
        Some(frac) if frac <= 0 => Some(Priorty::most_important()),
        Some(frac) if frac > 0 => Some(Priorty(frac as usize * task_inner.init_priorty.0)),
        _ => None,
    }
}
