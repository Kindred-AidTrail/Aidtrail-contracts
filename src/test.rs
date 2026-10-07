#![cfg(test)]

use crate::{
    types::{ProgramStatus, VendorStatus, VoucherIssueRequest},
    AidtrailContract, AidtrailContractClient,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String, Symbol, Vec,
};

pub struct TestFixture<'a> {
    pub env: Env,
    pub admin: Address,
    pub emergency_admin: Address,
    pub contract_id: Address,
    pub client: AidtrailContractClient<'a>,
    pub token_admin: Address,
    pub token_client: token::Client<'a>,
    pub token_admin_client: token::StellarAssetClient<'a>,
}

impl<'a> TestFixture<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let emergency_admin = Address::generate(&env);
        let contract_id = env.register_contract(None, AidtrailContract);
        let client = AidtrailContractClient::new(&env, &contract_id);

        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::Client::new(&env, &token_contract.address());
        let token_admin_client = token::StellarAssetClient::new(&env, &token_contract.address());

        client.initialize(&admin, &Some(emergency_admin.clone()));

        Self {
            env,
            admin,
            emergency_admin,
            contract_id,
            client,
            token_admin,
            token_client,
            token_admin_client,
        }
    }

    pub fn mint(&self, to: &Address, amount: i128) {
        self.token_admin_client.mint(to, &amount);
    }
}

#[test]
fn test_fixture_setup_works() {
    let f = TestFixture::setup();
    assert_eq!(f.client.get_admin(), Some(f.admin));
    assert_eq!(f.client.get_emergency_admin(), Some(f.emergency_admin));
    assert_eq!(f.client.is_paused(), false);
    assert_eq!(f.client.get_program_count(), 0);
}
