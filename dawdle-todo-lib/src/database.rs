use crate::data_center::{container::TaskContainer, TaskDataCenter};
use rusty_leveldb::{compressor, CompressorId, Options, DB};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
struct HistoryData {
    db_connection: DB,
}

impl HistoryData {
    pub async fn new(path: &str) -> Self {
        Self {
            // db_connection: Client::with_options({
            //     {
            //         let mut client_options = ClientOptions::parse(path)
            //             .await
            //             .expect("err at reading database url, check config!");
            //         client_options.app_name = Some("dawdle-todo-lib".to_owned());
            //         client_options
            //     }
            // })
            // .expect("connection error"),
            //
            db_connection: {
                let mut options = Options::default();
                options.compressor = compressor::SnappyCompressor::ID;
                DB::open(path, options).unwrap()
            },
        }
    }

    pub fn write_to_database(&mut self, task_database_id: String, database: TaskDataCenter) {
        let database_json = &database.to_json();
        self.db_connection.put(
            task_database_id.as_bytes(),
            database_json.to_string().as_bytes(),
        );
    }

    pub fn read_from_database(&mut self, task_database_id: String) -> TaskDataCenter {
        TaskDataCenter::from_database_json(
            serde_json::from_slice::<Value>(
                self.db_connection
                    .get(task_database_id.as_bytes())
                    .unwrap()
                    .as_slice(),
            )
            .expect("parse error while reading database for task_database"),
        )
    }
}
