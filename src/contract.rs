#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    state::{Config, CONFIG, REWARD_TOKENS},
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:refund_btc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &&Config {
            owner: info.sender.clone(),
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim {} => {
            let sender = info.sender;
            let rewards = REWARD_TOKENS
                .load(deps.storage, sender.clone())
                .unwrap_or(vec![]);
            return Ok(Response::new().add_message(BankMsg::Send {
                to_address: sender.to_string(),
                amount: rewards,
            }));
        }
        ExecuteMsg::AddRewarders { rewarders, rewards } => {
            let config = CONFIG.load(deps.storage)?;
            let owner = config.owner;
            if owner != info.sender {
                return Err(ContractError::Unauthorized {});
            }
            if rewarders.len() != rewards.len() {
                return Err(ContractError::InvalidArguments {});
            }
            for (i, rewarder) in rewarders.iter().enumerate() {
                let mut store_rewards = REWARD_TOKENS
                    .load(deps.storage, rewarder.clone())
                    .unwrap_or(vec![]);
                let rewards_arr = rewards.get(i).unwrap();
                for reward_item in rewards_arr.iter() {
                    store_rewards.push(reward_item.clone());
                }
                REWARD_TOKENS.save(deps.storage, rewarder.clone(), &store_rewards)?;
            }
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RewardTokens { addr } => {
            let rewards = REWARD_TOKENS.load(deps.storage, addr).unwrap_or(vec![]);
            to_json_binary(&rewards)
        }
        QueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let original_version =
        cw2::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("new_version", original_version.to_string()))
}
