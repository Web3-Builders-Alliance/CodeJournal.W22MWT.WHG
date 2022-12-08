//we're modifies here our contract

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
    let owner = msg.owner.and_then(|addr_string|deps.api.addr_validate(addr_string.as_str()).ok()).unwrap_or(info.sender);

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
        ExecuteMsg::NewEntry { description, priority } => unimplemented!(),
        ExecuteMsg::UpdateEntry { id, description, status, priority } =>unimplemented!(),
        ExecuteMsg::DeleteEntry { id } => unimplemented!(),
    };
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
        LIST.save(deps.storage, id, &new_entry);
        Ok(Response::new().add_attribute("method", "execute_new_entry").add_attribute("new_entry_id", id.to_string()))

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
        Ok(Response::new().add_attribute("method", "execute_update_entry")
                        .add_attribute("updated_entry_id", id.to_string()))
        }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
