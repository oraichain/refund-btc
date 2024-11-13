use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use oraiswap::asset::Asset;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REWARD_TOKENS: Map<Addr, Vec<Asset>> = Map::new("reward_tokens");
