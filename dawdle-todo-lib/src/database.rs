use std::{collections::HashMap, sync::Arc};

use mongodb::{
    bson::{self, Bson},
    error::Error,
    options::ClientOptions,
    Client,
};

use crate::data_center::container::TaskContainer;
struct HistoryData {
    db_connection: Client,
}

impl HistoryData {
    pub async fn new(path: &str) -> Self {
        Self {
            db_connection: Client::with_options({
                {
                    let mut client_options = ClientOptions::parse(path)
                        .await
                        .expect("err at reading database url, check config!");
                    client_options.app_name = Some("dawdle-todo-lib".to_owned());
                    client_options
                }
            })
            .expect("connection error"),
        }
    }

    pub fn read_from_database(
        &mut self,
        task_database_id: String,
    ) -> HashMap<String, Arc<dyn TaskContainer>> {
        let db = self.db_connection.database("dawdle_todo_cache");
        let collection = db.collection::<bson::Document>(&task_database_id);
        //TODO 使用bson
        todo!()
    }
}
