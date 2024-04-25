use core::panic;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use toml::value::Datetime;

use crate::{
    configurations::TaskConfigRoot,
    data_center::{
        container::{
            basic_priority_queue::BasicPriorityContainer, once::OnceContainer, TaskContainer,
        },
        task::{Priority, Task},
    },
};

use super::TaskDataCenter;

#[test]
fn test_data_center() {
    // let example_config_root = TaskConfigRoot {
    //     id: "this_is_a_example_config_of_task_group".to_owned(),
    //     tasks: (0..3)
    //         .map(|i| Task {
    //             id: format!("task{i}").to_string(),
    //             init_priority: Priority::most_important(),
    //             complete_time: 114514,
    //             begin_date: Datetime::from_str("2024-04-24").unwrap(),
    //             end_date: Datetime::from_str("2024-04-24").unwrap(),
    //         })
    //         .collect(),
    //     defined_containers: {
    //         let mut hash_map = HashMap::new();
    //         (0..3)
    //             .map(|i| OnceContainer::new(format!("task{i}")))
    //             .for_each(|v| {
    //                 hash_map.insert(v.id().to_string(), serde_json::to_string(&v).unwrap());
    //             });
    //         hash_map.insert(
    //             "basic_priority_queue_example".to_string(),
    //             serde_json::to_string(&BasicPriorityContainer::new(
    //                 "basic_priority_queue_example".to_string(),
    //                 vec!["task0", "task1", "task2"]
    //                     .iter()
    //                     .map(|s| s.to_string())
    //                     .collect(),
    //                 None,
    //             ))
    //             .unwrap(),
    //         );
    //         hash_map
    //     },
    // };

    // TaskDataCenter::init(example_config_root)
    //     .container_list
    //     .values()
    //     .for_each(|a| {
    //         println!(
    //             "{}:weak-{},strong-{}",
    //             a.read().unwrap().id(),
    //             Arc::weak_count(a),
    //             Arc::strong_count(a)
    //         )
    //     });

    // panic!()
}
