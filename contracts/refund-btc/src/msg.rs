use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use oraiswap::asset::Asset;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Claim {},
    AddRewarder { rewarder: Addr, rewards: Vec<Asset> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Asset>)]
    RewardTokens { addr: Addr },
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
