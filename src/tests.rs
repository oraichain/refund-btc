use cosmwasm_std::{coin, coins, testing::mock_dependencies, Addr, Coin};

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

    let config: Config = app
        .query(contract_addr.clone(), &crate::msg::QueryMsg::Config {})
        .unwrap();
    assert_eq!(config.owner.to_string(), owner.clone());

    // add rewarder
    app.execute(
        Addr::unchecked(owner.clone()),
        contract_addr.clone(),
        &crate::msg::ExecuteMsg::AddRewarders {
            rewarders: vec![Addr::unchecked(alice.clone())],
            rewards: vec![vec![coin(100, "orai"), coin(50, "obtc")]],
        },
        &[],
    )
    .unwrap();

    // query rewarder token
    let reward_tokens: Vec<Coin> = app
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
}
