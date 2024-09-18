pub mod contract;
pub mod msg;

pub use crate::contract::{Cw721Contract, ContractError};
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, NftDetailsResponse};

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: contract::InstantiateMsg,
    ) -> Result<Response, ContractError> { // Changed return type to ContractError
        let tract = Cw721Contract::default();
        tract.instantiate(deps, env, info, msg) // Return ContractError directly
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: contract::ExecuteMsg, // Changed to use contract::ExecuteMsg
    ) -> Result<Response, ContractError> {
        let tract = Cw721Contract::default();
        tract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: contract::QueryMsg) -> StdResult<Binary> { // Changed to use contract::QueryMsg
        let tract = Cw721Contract::default();
        tract.query(deps, env, msg)
    }
}