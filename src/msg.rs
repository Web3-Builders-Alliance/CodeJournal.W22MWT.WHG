use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Priority, Status};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Option<String> //it's optional, when it's not provided that will be assigned as the owner by default
                              //we're using invalidated String address which should be validated by contract 
                              //Option can be Some or None
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
//the owner should able to modify the list 
pub enum ExecuteMsg {
    NewEntry{ //add new Entry to do list
        description: String,
        priority: Option<Priority>,
    },
    UpdateEntry{ //update an exist entry
        id: u64,
        description: Option<String>,
        status: Option<Status>,
        priority: Option<Priority>,
    }, 
    DeleteEntry{ //delete an exist entry 
        id: u64,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
