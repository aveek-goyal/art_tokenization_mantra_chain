use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary, Uint128, Coin, StdError};
use cw2::set_contract_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Item;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub max_mints: u64,
    pub mint_price: Coin,
    pub token_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    NftDetails {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct NftDetailsResponse {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub max_mints: u64,
    pub mint_price: Coin,
    pub token_uri: Option<String>,
    pub token_count: u64,
}

pub struct Cw721Contract<'a> {
    pub contract_info: Item<'a, NftDetailsResponse>,
    pub token_count: Item<'a, u64>,
}

impl<'a> Default for Cw721Contract<'a> {
    fn default() -> Self {
        Self {
            contract_info: Item::new("contract_info"),
            token_count: Item::new("token_count"),
        }
    }
}

impl<'a> Cw721Contract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = NftDetailsResponse {
            name: msg.name,
            symbol: msg.symbol,
            minter: deps.api.addr_validate(&msg.minter)?.to_string(),
            max_mints: msg.max_mints,
            mint_price: msg.mint_price,
            token_uri: msg.token_uri,
            token_count: 0,
        };
        self.contract_info.save(deps.storage, &info)?;
        self.token_count.save(deps.storage, &0u64)?;

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint {} => self.mint(deps, env, info),
        }
    }

    pub fn mint(&self, deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let mut contract_info = self.contract_info.load(deps.storage)?;
        let mut token_count = self.token_count.load(deps.storage)?;

        if token_count >= contract_info.max_mints {
            return Err(ContractError::MaxMintsReached {});
        }

        if info.funds.len() != 1 || info.funds[0] != contract_info.mint_price {
            return Err(ContractError::IncorrectPayment {});
        }

        token_count += 1;
        self.token_count.save(deps.storage, &token_count)?;

        contract_info.token_count = token_count;
        self.contract_info.save(deps.storage, &contract_info)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("token_id", token_count.to_string()))
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::NftDetails {} => to_json_binary(&self.contract_info.load(deps.storage)?),
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Max mints reached")]
    MaxMintsReached {},

    #[error("Incorrect payment amount")]
    IncorrectPayment {},
}