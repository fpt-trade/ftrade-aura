use crate::state::Employee;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub career: String,
    pub domain: String,
    pub group: String,
    pub number_of_course: i32,
    pub department: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddNew {
        id: String,
        name: String,
        career: String,
        domain: String,
        group: String,
        number_of_course: i32,
        department: String,
        position: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetEmployee returns the employee's information
    GetEmployee { id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EmployeeInfoResponse {
    pub employee: Option<Employee>,
}
