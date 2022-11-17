use my_no_sql_server_abstractions::MyNoSqlEntity;
use serde::{Serialize, Deserialize};

pub const TEST_DATA_NOSQL_TABLE_NAME: &str = "test-data";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestDataNoSqlEntity {
    #[serde(rename = "RowKey")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
}

impl MyNoSqlEntity for TestDataNoSqlEntity {
    fn get_partition_key(&self) -> &str {
        "td"
    }

    fn get_row_key(&self) -> &str {
        &self.id
    }

    fn get_time_stamp(&self) -> i64 {
        self.time_stamp.parse::<i64>().unwrap()
    }
}