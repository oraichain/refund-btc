use crate::msg::{self};
use cosmwasm_std::{testing::mock_env, Env, Timestamp};
use cosmwasm_std::{Addr, Coin};
use cosmwasm_testing_util::MockResult;
use derive_more::{Deref, DerefMut};

pub type TestMockApp = cosmwasm_testing_util::MultiTestMockApp;
#[derive(Deref, DerefMut)]
pub struct MockApp {
    #[deref]
    #[deref_mut]
    app: TestMockApp,
    contract_id: u64,
}

#[allow(dead_code)]
impl MockApp {
    pub fn new(init_balances: &[(&str, &[Coin])]) -> (Self, Vec<String>) {
        let (mut app, accounts) = TestMockApp::new(init_balances);
        let contract_id = app.upload(Box::new(
            cosmwasm_testing_util::ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            ),
        ));
        (Self { app, contract_id }, accounts)
    }

    /// external method
    pub fn create_contract(
        &mut self,
        sender: Addr,
        init_msg: &msg::InstantiateMsg,
    ) -> MockResult<Addr> {
        let code_id = self.contract_id;
        let addr = self.instantiate(code_id, sender.clone(), init_msg, &[], "cw-bitcoin-bridge")?;
        Ok(addr)
    }
}
