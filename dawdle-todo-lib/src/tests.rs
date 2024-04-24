use chrono::DateTime;
use toml::value::Datetime;

use crate::data_center::task::{InternalDate, Priority, Task};

#[test]
fn task_serialize() {
    let task = Task {
        id: "mulimomuli".to_owned(),
        init_priority: Priority(114),
        complete_time: 514,
        begin_date: InternalDate::current_time().0,
        end_date: InternalDate::current_time().0,
    };
    println!("{}", serde_json::to_string(&task))
}
