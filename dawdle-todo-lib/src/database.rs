use diesel::prelude::*;
struct HistoryData {
    sql_connection: PgConnection,
}

impl HistoryData {
    pub fn new(path: &str) -> Self {
        Self {
            sql_connection: PgConnection::establish(path)
                .expect("err at reading database url, check config!"),
        }
    }

    pub fn operate(&mut self) {
        // self.sql_connection.
        //TODO Design task structures and sql structures
    }
}
