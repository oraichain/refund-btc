use cosmwasm_std::{coin, coins, Addr};
use oraiswap::asset::{Asset, AssetInfo};

use crate::{helper::MockApp, state::Config};
use cosmwasm_std::Uint128;

#[test]
fn test_simple_flows() {
    let (mut app, accounts) = MockApp::new(&[
        (
            "perfogic",
            &vec![coin(100_000_000_000, "orai"), coin(50_000_000, "obtc")],
        ),
        ("alice", &coins(1_000_000, "orai")),
    ]);
    let owner = accounts[0].clone();
    let alice = accounts[1].clone();
    let contract_addr = app
        .create_contract(
            Addr::unchecked(owner.clone()),
            &crate::msg::InstantiateMsg {},
        )
        .unwrap();
    let usdt_addr = app.create_token(&owner, "usdt", 500_000_000);

    let config: Config = app
        .query(contract_addr.clone(), &crate::msg::QueryMsg::Config {})
        .unwrap();
    assert_eq!(config.owner.to_string(), owner.clone());

    // add rewarder
    app.execute(
        Addr::unchecked(owner.clone()),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::AddRewarder {
            rewarder: Addr::unchecked(alice.clone()),
            rewards: vec![
                Asset {
                    info: AssetInfo::NativeToken {
                        denom: "orai".to_string(),
                    },
                    amount: Uint128::new(100),
                },
                Asset {
                    info: AssetInfo::NativeToken {
                        denom: "obtc".to_string(),
                    },
                    amount: Uint128::new(50),
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: usdt_addr.clone(),
                    },
                    amount: Uint128::new(100),
                },
            ],
        },
        &[],
    )
    .unwrap();

    // query rewarder token
    let reward_tokens: Vec<Asset> = app
        .query(
            contract_addr.clone(),
            &crate::msg::QueryMsg::RewardTokens {
                addr: Addr::unchecked(alice.clone()),
            },
        )
        .unwrap();
    println!("{:?}", reward_tokens);

    // fund the contract
    app.send_coins(
        Addr::unchecked(owner.clone()),
        contract_addr.clone(),
        &vec![coin(100, "orai"), coin(50, "obtc")],
    )
    .unwrap();

    app.execute(
        Addr::unchecked(owner.clone()),
        usdt_addr.clone(),
        &cw20_base::msg::ExecuteMsg::Transfer {
            recipient: contract_addr.clone().to_string(),
            amount: Uint128::new(100),
        },
        &[],
    )
    .unwrap();

    // claim the fund
    app.execute(
        Addr::unchecked(alice.clone()),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::Claim {},
        &[],
    )
    .unwrap();

    // check balance
    let balance: Uint128 = app
        .query_balance(Addr::unchecked(alice.clone()), "orai".to_string())
        .unwrap();
    assert_eq!(balance, Uint128::new(1000100));
    let balance: Uint128 = app
        .query_balance(Addr::unchecked(alice.clone()), "obtc".to_string())
        .unwrap();
    assert_eq!(balance, Uint128::new(50));
    let balance: cw20::BalanceResponse = app
        .query(
            usdt_addr.clone(),
            &cw20_base::msg::QueryMsg::Balance {
                address: alice.clone().to_string(),
            },
        )
        .unwrap();
    assert_eq!(balance.balance, Uint128::new(100));
}
