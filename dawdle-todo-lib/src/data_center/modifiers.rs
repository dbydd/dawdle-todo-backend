extern crate toml;

use crate::data_center::task::InternalDate;

use super::{container::TaskContainer, task::Priority, TaskDataCenter};

pub fn simple_in_time_complete(
    taskinner: &dyn TaskContainer,
    center: &TaskDataCenter,
) -> Option<Priority> {
    let task_inner = taskinner.peek_task_inner(center);
    let times_remain = taskinner.times_remain(center);
    let total_range = InternalDate(task_inner.end_date) - InternalDate(task_inner.begin_date);
    match Some(times_remain.num_hours() / total_range.num_hours()) {
        Some(frac) if frac <= 0 => Some(Priority::most_important()),
        Some(frac) if frac > 0 => Some(Priority(frac as usize * task_inner.init_priority.0)),
        _ => None,
    }
}
