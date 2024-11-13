use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use oraiswap::asset::Asset;

use crate::state::Config;

#[cw_serde]
pub struct RewardTokensResponse {
    pub reward_tokens: Vec<Coin>,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Claim {},
    AddRewarders {
        rewarders: Vec<Addr>,
        rewards: Vec<Vec<Asset>>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RewardTokensResponse)]
    RewardTokens { addr: Addr },
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
