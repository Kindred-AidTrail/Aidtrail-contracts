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

#[test]
fn test_create_program_and_getters() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://program-alpha");

    let p1 = f.client.create_program(&ngo, &f.token_client.address, &meta);
    assert_eq!(p1, 1);
    assert_eq!(f.client.get_program_count(), 1);

    let prog = f.client.get_program(&p1);
    assert_eq!(prog.id, 1);
    assert_eq!(prog.ngo, ngo);
    assert_eq!(prog.status, ProgramStatus::Active);
    assert_eq!(prog.total_funded, 0);
    assert_eq!(prog.total_released, 0);
    assert_eq!(prog.total_allocated, 0);

    // Query non-existent program
    assert!(f.client.try_get_program(&999).is_err());
}

#[test]
fn test_add_milestone_and_validations() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    let v1 = Address::generate(&f.env);
    let v2 = Address::generate(&f.env);
    let mut verifiers = Vec::new(&f.env);
    verifiers.push_back(v1.clone());
    verifiers.push_back(v2.clone());

    let m_desc = String::from_str(&f.env, "ipfs://milestone-1");

    // Success: 2 verifiers, 2 required approvals, 1,000 stroops
    let m_id = f.client.add_milestone(&ngo, &p_id, &1_000, &m_desc, &2, &verifiers);
    assert_eq!(m_id, 1);
    assert_eq!(f.client.get_milestone_count(&p_id), 1);

    let milestone = f.client.get_milestone(&p_id, &m_id);
    assert_eq!(milestone.id, 1);
    assert_eq!(milestone.amount, 1_000);
    assert_eq!(milestone.required_approvals, 2);
    assert_eq!(milestone.verifiers.len(), 2);
    assert_eq!(milestone.approvals.len(), 0);

    // Fail: unauthorized caller
    let rando = Address::generate(&f.env);
    assert!(f.client.try_add_milestone(&rando, &p_id, &500, &m_desc, &1, &verifiers).is_err());

    // Fail: invalid amount <= 0
    assert!(f.client.try_add_milestone(&ngo, &p_id, &0, &m_desc, &1, &verifiers).is_err());
    assert!(f.client.try_add_milestone(&ngo, &p_id, &-10, &m_desc, &1, &verifiers).is_err());

    // Fail: required_approvals > verifiers.len()
    assert!(f.client.try_add_milestone(&ngo, &p_id, &500, &m_desc, &3, &verifiers).is_err());

    // Fail: required_approvals == 0
    assert!(f.client.try_add_milestone(&ngo, &p_id, &500, &m_desc, &0, &verifiers).is_err());

    // Fail: duplicate verifier in list
    let mut dup_verifiers = Vec::new(&f.env);
    dup_verifiers.push_back(v1.clone());
    dup_verifiers.push_back(v1.clone());
    assert!(f.client.try_add_milestone(&ngo, &p_id, &500, &m_desc, &2, &dup_verifiers).is_err());
}

#[test]
fn test_fund_program_transfers_and_tracks_donor() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog-fund");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    let donor = Address::generate(&f.env);
    f.mint(&donor, 50_000);

    // Initial funding of 20,000
    f.client.fund_program(&donor, &p_id, &20_000);

    assert_eq!(f.token_client.balance(&f.contract_id), 20_000);
    assert_eq!(f.token_client.balance(&donor), 30_000);

    let prog = f.client.get_program(&p_id);
    assert_eq!(prog.total_funded, 20_000);

    let contrib = f.client.get_donor_contribution(&p_id, &donor).unwrap();
    assert_eq!(contrib.amount, 20_000);
    assert_eq!(contrib.refunded, false);

    // Subsequent funding of 10,000 accumulates
    f.client.fund_program(&donor, &p_id, &10_000);
    assert_eq!(f.token_client.balance(&f.contract_id), 30_000);
    assert_eq!(f.token_client.balance(&donor), 20_000);

    let contrib2 = f.client.get_donor_contribution(&p_id, &donor).unwrap();
    assert_eq!(contrib2.amount, 30_000);
    assert_eq!(f.client.get_program(&p_id).total_funded, 30_000);

    // Invalid amounts fail
    assert!(f.client.try_fund_program(&donor, &p_id, &0).is_err());
    assert!(f.client.try_fund_program(&donor, &p_id, &-500).is_err());
}



