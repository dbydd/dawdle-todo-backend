use std::{collections::HashMap, str::FromStr};

use chrono::DateTime;
use log::log;
use serde_json::json;
use toml::value::Datetime;

use crate::{
    configurations::TaskConfigRoot,
    data_center::{
        task::{InternalDate, Priority, Task},
        TaskDataCenter,
    },
};

#[test]
fn task_serialize() {
    let task = Task {
        id: "mulimomuli".to_owned(),
        init_priority: Priority(114),
        complete_time: 514,
        begin_date: Datetime::from_str("2024-04-24"),
        end_date: Datetime::from_str("2024-04-24"),
    };
    // println!("{}", serde_json::to_string_pretty(&task).unwrap())
    assert_eq!(
        json!({
          "id": "mulimomuli",
          "init_priority": 114,
          "complete_time": 514,
          "begin_date": {
            "$__toml_private_datetime": "2024-04-24"
          },
          "end_date": {
            "$__toml_private_datetime": "2024-04-24"
          }
        }),
        serde_json::to_value(task).unwrap()
    )
}

#[test]
fn test_task_data_center_and_config_root() {
    let example_config_root = TaskConfigRoot {
        id: "this_is_a_example_config_of_task_group".to_owned(),
        tasks: (0..3)
            .map(|i| Task {
                id: format!("task{i}").to_string(),
                init_priority: Priority::most_important(),
                complete_time: "114514",
                begin_date: Datetime::from_str("2024-04-24"),
                end_date: Datetime::from_str("2024-04-24"),
            })
            .collect(),
        defined_containers: {
            let hash_map = HashMap::new();
            // hash_map.insert("container_a", )
            todo!("加上标注")
        },
    };
    example_config_root
}
