use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

use crate::msg::{EntryResponse, ExecuteMsg, QueryMsg};  //Changed to EntryResponse

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this. Rename it to your contract name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Get Custom
    pub fn custom_query<Q, T, CQ>(&self, querier: &Q, val: String) -> StdResult<EntryResponse>  //changed EntryResponse
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::QueryEntry { id: (0u64) }; //we should changed 
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: EntryResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?; //changed to EntryResponse
        Ok(res)
    }
}
