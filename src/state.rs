use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static STORE_KEY: &[u8] = b"store";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub career: String,
    pub domain: String,
    pub group: String,
    pub number_of_course: i32,
    pub department: String,
    pub position: String,
}

pub fn store(storage: &mut dyn Storage) -> Bucket<Employee> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Employee> {
    bucket_read(storage, STORE_KEY)
}
