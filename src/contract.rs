//we're modifies here our contract

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order};  //add Order we use it in query list fn
use cw2::set_contract_version;
use cw_storage_plus::Bound; //import Bound and use query list fn


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, EntryResponse, ListResponse};
use crate::state::{Config, CONFIG, ENTRY_SEQ, Entry, Priority, LIST, Status}; //import from state.rs 
use std::ops::Add; //import and use when we create a new Entry 

//this store information for migration
const CONTRACT_NAME: &str = "crates.io:to-do-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
//this will be called once before the contract is executed
//we can set configuration that never be modified by other call
//this entry point to instantieate the contract
pub fn instantiate(
    deps: DepsMut,  //removed the underscore
    _env: Env,      //we use underscore because we don't use here
    info: MessageInfo, //
    msg: InstantiateMsg, //this is from msg.rs (struct)
) -> Result<Response, ContractError> { //this will return responde or Error
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; 
    //owner was optional so if the instantieat msg contains an owner address, this is validate and use it, otherwise the owner is the address that instatietes the contract
    let owner = msg
        .owner
        .and_then(|addr_string|deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config { 
        owner: owner.clone(),
    };
    CONFIG.save(deps.storage, &config)?; //saving the owner address to contract storage

    ENTRY_SEQ.save(deps.storage, &0u64)?; //starting with 0, save the entry sequence 

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg, //it's enum we should check every possibility
) ->Result<Response, ContractError> {
    match msg { 
        ExecuteMsg::NewEntry { description, priority } => execute_new_entry(deps, info, description, priority),
        ExecuteMsg::UpdateEntry { id, description, status, priority } => execute_update_entry(deps, info, id, description, status, priority),
        ExecuteMsg::DeleteEntry { id } => execute_delete_entry(deps, info, id),
    }
}
    //create a fn for newentry

    fn execute_new_entry(
        deps: DepsMut,
        info: MessageInfo,
        description : String,
        priority: Option<Priority>
    ) -> Result<Response, ContractError>{
        let owner = CONFIG.load(deps.storage)?.owner; //check message sender is the owner of the contract

        if info.sender != owner{ //if not the owner returns error 
            return Err(ContractError::Unauthorized {}); //create a new error message in error.rs
        }
        //we generate an unique id for the new entry and save it to the contract storage
        let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| {
            Ok(id.add(1))
        })?;
        //status set ToDo and priority None by default 
        let new_entry = Entry{
            id,
            description,
            priority: priority.unwrap_or(Priority::None),
            status: Status::ToDo
        
        };
        //save in the List with matching id and return Responde with the attribute
        LIST.save(deps.storage, id, &new_entry)?;
        
        Ok(Response::new()
            .add_attribute("method", "execute_new_entry")
            .add_attribute("new_entry_id", id.to_string()))

    }

    fn execute_update_entry(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
        description: Option<String>,
        status: Option<Status>,
        priority: Option<Priority>
    ) -> Result<Response, ContractError>{
        let owner = CONFIG.load(deps.storage)?.owner; //check the sender is the owner of the contract
        if info.sender != owner {
            return Err(ContractError::Unauthorized {}); //return error
        }
        //load entry with mathcing id from List
        let entry = LIST.load(deps.storage, id)?;
        
        // If optional parameters are not 
        // provided, the function defaults back to the value from the entry loaded.
        let updated_entry = Entry {
            id,
            description: description.unwrap_or(entry.description),
            status: status.unwrap_or(entry.status),
            priority: priority.unwrap_or(entry.priority),
        };
        // saves the updated entry to the `LIST` with the matching `id` and returns a `Response` 
        // with the relevant attributes.
        LIST.save(deps.storage, id, &updated_entry)?;
        Ok(Response::new()
            .add_attribute("method", "execute_update_entry")
            .add_attribute("updated_entry_id", id.to_string()))
    }

    fn execute_delete_entry(
        deps: DepsMut,
        info: MessageInfo,
        id: u64,
    ) -> Result<Response, ContractError>{
        let owner = CONFIG.load(deps.storage)?.owner; //check the sender is the owner of the contrcat
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  }); // if not the owner, return error
        }
        //we can remove the entry
        LIST.remove(deps.storage, id);  //remove entry with the matching id
        //returns a response with the relevant attributes
        Ok(Response::new().add_attribute("method", "execute_delete_entry").add_attribute("deleted_entry_id", id.to_string()))
    }


#[cfg_attr(not(feature = "library"), entry_point)]
//quering entries or subset of the whole list
// the query fn matches the received QueryMsg and returns a query responde in byte-array format
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryEntry { id :_} => unimplemented!(),
        QueryMsg::QueryList { start_after:_, limit:_ } => unimplemented!(),
    }
}
//create function to query entry
//will list entry with the matching id
fn _query_entry(
    deps: Deps,
    id: u64,
) -> StdResult<EntryResponse>{ 
    let entry = LIST.load(deps.storage, id)?; //load the entry with the id

    //it will return  EntryResponse with the attributes of the loaded entry 
    Ok(EntryResponse { id: entry.id, description: entry.description, status: entry.status, priority: entry.priority })
}
//will query the whole list 
//add a limit for the custom range query , limits for pagination
const MAX_LIMIT:u32 = 30;
const DEFAULT_LIMIT:u32 = 10;

fn _query_list(
    deps: Deps, 
    start_after:Option<u64>, //it defines the subset of the list in order to limit the numberof entries returned
    limit: Option<u32>,
)->StdResult<ListResponse>{
    let start = start_after.map(Bound::exclusive);  //start_after serves as the lower index bound for the function
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize; //determins the max number of entries to be returned

    let entries:StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Ascending) //outputs the resultas a vector of(id, Entry) tuples
        .take(limit)                                                           //outputs the resultas a vector of(id, Entry) tuples
        .collect();                                                                                           //outputs the resultas a vector of(id, Entry) tuples
    
    let result = ListResponse {
        entries: entries?.into_iter().map(|l|l.1.into()).collect(), //this will be returned as query response
    };
    Ok(result)
}

#[cfg(test)]
mod tests {}
