use std::str::FromStr;

use serde_json::json;

use crate::data_center::task::InternalDate;

use super::{basic_priority_queue::BasicPriorityContainer, once::OnceContainer};

#[test]
fn once_container_serialize() {
    let once_container = OnceContainer::new("mulimomuli".to_string());
    assert_eq!(
        json!(
            {
                "id": "mulimomuli",
                "current_time": 0
            }
        ),
        serde_json::to_value(once_container).unwrap()
    )
}

#[test]
fn basic_priority_queue_container_serialize() {
    let basic_priority_container = BasicPriorityContainer::new(
        "114514".to_owned(),
        { (0..3).map(|i| format!("subtask{i}")).to_owned() }.collect(),
        None,
    );
    assert_eq!(
        json!({
          "id": "114514",
          "task_queue": [
            "subtask0",
            "subtask1",
            "subtask2"
          ],
          "init_priority": 0
        }),
        serde_json::to_value(basic_priority_container).unwrap()
    )
}
