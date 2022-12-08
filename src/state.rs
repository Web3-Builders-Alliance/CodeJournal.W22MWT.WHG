//storing contract data 
use std::process::ExitStatus;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map}; // we should import Map 


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr, //address stored here
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry { //to do 
    pub id: u64,
    pub description: String, 
    pub status: Status, 
    pub priority: Priority,
    
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status{
    ToDo,
    InProgress,
    Done,
    Cancelled
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Priority{
    None,
    Low,
    Medium,
    High
}
//creating constant to store varieble on the chain
//item stores (as a single varieble) by storage key
pub const CONFIG: Item<Config> = Item::new("config");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq"); //entry stored as entry_seq 
pub const LIST: Map<u64, Entry> = Map::new("list"); //it's a map with a key: id and value: entry
