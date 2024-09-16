use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use cosmwasm_std::{Addr, BlockInfo, StdResult, Storage, Coin};

use cw721::{ContractInfoResponse, CustomMsg, Cw721, Expiration};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub struct Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub token_count: Item<'a, u64>,
    pub token_uri: Item<'a, Option<String>>, 
    pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a,T>>,
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub(crate) _custom_response: PhantomData<C>,
    pub minting_allowed: Item<'a, bool>,
    pub max_mints: Item<'a, u64>,
    pub mint_price: Item<'a, Coin>,
}

// This is a signal, the implementations are in other files
impl<'a, T, C> Cw721<T, C> for Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

impl<T, C> Default for Cw721Contract<'static, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    fn default() -> Self {
        Self {
            contract_info: Item::new("contract_info"),
            minter: Item::new("minter"),
            token_count: Item::new("num_tokens"),
            tokens: IndexedMap::new("tokens", TokenIndexes {
                owner: MultiIndex::new(token_owner_idx, "tokens", "tokens__owner"),
            }),
            operators: Map::new("operators"),
            _custom_response: PhantomData,
            minting_allowed: Item::new("minting_allowed"),
            max_mints: Item::new("max_mints"),
            mint_price: Item::new("mint_price"),
            token_uri: Item::new("token_uri"),
        }
    }
}

impl<'a, T, C> Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    fn new(
        contract_key: &'a str,
        minter_key: &'a str,
        token_count_key: &'a str,
        operator_key: &'a str,
        tokens_key: &'a str,
        tokens_owner_key: &'a str,
    ) -> Self {
        let indexes = TokenIndexes {
            owner: MultiIndex::new(token_owner_idx, tokens_key, tokens_owner_key),
        };
        Self {
            contract_info: Item::new(contract_key),
            minter: Item::new(minter_key),
            token_count: Item::new(token_count_key),
            operators: Map::new(operator_key),
            tokens: IndexedMap::new(tokens_key, indexes),
            _custom_response: PhantomData,
            minting_allowed: Item::new("minting_allowed"),
            max_mints: Item::new("max_mints"),
            mint_price: Item::new("mint_price"),
            token_uri: Item::new("token_uri"),
        }
    }

    pub fn token_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.token_count.may_load(storage)?.unwrap_or_default())
    }

    /// Update the token count by incrementing or decrementing it.
    pub fn update_token_count(&self, storage: &mut dyn Storage, increment: bool) -> StdResult<u64> {
        let mut current_count = self.token_count(storage)?;
        if increment {
            current_count += 1;
        } else {
            current_count -= 1;
        }
        self.token_count.save(storage, &current_count)?;
        Ok(current_count)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

pub struct TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, Addr, TokenInfo<T>, Addr>,
}

impl<'a, T> IndexList<TokenInfo<T>> for TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo<T>>> + '_> {
        let v: Vec<&dyn Index<TokenInfo<T>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx<T>(d: &TokenInfo<T>) -> Addr {
    d.owner.clone()
}
