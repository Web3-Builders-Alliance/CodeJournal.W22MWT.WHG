//we're modifies here our contract

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, ENTRY_SEQ}; //import from state.rs 

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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
