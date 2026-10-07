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

#[test]
fn test_cannot_reinitialize() {
    let f = TestFixture::setup();
    let res = f.client.try_initialize(&f.admin, &None);
    assert!(res.is_err());
}

#[test]
fn test_emergency_pause_and_unpause() {
    let f = TestFixture::setup();

    // Admin can pause
    f.client.set_paused(&f.admin, &true);
    assert_eq!(f.client.is_paused(), true);

    // Emergency admin can unpause
    f.client.set_paused(&f.emergency_admin, &false);
    assert_eq!(f.client.is_paused(), false);

    // Emergency admin can pause
    f.client.set_paused(&f.emergency_admin, &true);
    assert_eq!(f.client.is_paused(), true);

    // Unauthorized user cannot toggle pause
    let rando = Address::generate(&f.env);
    let res = f.client.try_set_paused(&rando, &false);
    assert!(res.is_err());
    assert_eq!(f.client.is_paused(), true);
}

#[test]
fn test_transfer_admin_and_emergency_admin() {
    let f = TestFixture::setup();
    let new_admin = Address::generate(&f.env);
    let new_emergency = Address::generate(&f.env);

    // Non-admin cannot transfer admin
    let rando = Address::generate(&f.env);
    assert!(f.client.try_set_admin(&rando, &new_admin).is_err());

    // Current admin transfers ownership
    f.client.set_admin(&f.admin, &new_admin);
    assert_eq!(f.client.get_admin(), Some(new_admin.clone()));

    // Old admin can no longer transfer
    assert!(f.client.try_set_admin(&f.admin, &rando).is_err());

    // New admin updates emergency admin
    f.client.set_emergency_admin(&new_admin, &Some(new_emergency.clone()));
    assert_eq!(f.client.get_emergency_admin(), Some(new_emergency));
}

#[test]
fn test_paused_blocks_program_creation() {
    let f = TestFixture::setup();
    f.client.set_paused(&f.admin, &true);

    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://meta");
    let res = f.client.try_create_program(&ngo, &f.token_client.address, &meta);
    assert!(res.is_err());
}

