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

#[test]
fn test_multi_verifier_approval_consensus() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog-m-of-n");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    let v1 = Address::generate(&f.env);
    let v2 = Address::generate(&f.env);
    let v3 = Address::generate(&f.env);
    let mut verifiers = Vec::new(&f.env);
    verifiers.push_back(v1.clone());
    verifiers.push_back(v2.clone());
    verifiers.push_back(v3.clone());

    let m_desc = String::from_str(&f.env, "ipfs://m1");
    let m_id = f.client.add_milestone(&ngo, &p_id, &5_000, &m_desc, &2, &verifiers);

    let ev1 = String::from_str(&f.env, "ipfs://evidence1");
    let ev2 = String::from_str(&f.env, "ipfs://evidence2");

    // Verifier 1 approves (1 of 2): status still Pending
    f.client.approve_milestone(&v1, &p_id, &m_id, &ev1);
    let m_state = f.client.get_milestone(&p_id, &m_id);
    assert_eq!(m_state.approvals.len(), 1);
    assert_eq!(m_state.status, crate::types::MilestoneStatus::Pending);
    assert_eq!(f.client.is_milestone_approved(&p_id, &m_id), false);

    // Duplicate approval by Verifier 1 fails
    assert!(f.client.try_approve_milestone(&v1, &p_id, &m_id, &ev1).is_err());

    // Unauthorized verifier fails
    let rando = Address::generate(&f.env);
    assert!(f.client.try_approve_milestone(&rando, &p_id, &m_id, &ev1).is_err());

    // Verifier 2 approves (2 of 2): status transitions to Approved!
    f.client.approve_milestone(&v2, &p_id, &m_id, &ev2);
    let m_approved = f.client.get_milestone(&p_id, &m_id);
    assert_eq!(m_approved.approvals.len(), 2);
    assert_eq!(m_approved.status, crate::types::MilestoneStatus::Approved);
    assert_eq!(f.client.is_milestone_approved(&p_id, &m_id), true);
}

#[test]
fn test_release_milestone_success_and_invariants() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog-rel");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    let v1 = Address::generate(&f.env);
    let mut verifiers = Vec::new(&f.env);
    verifiers.push_back(v1.clone());

    let m_desc = String::from_str(&f.env, "ipfs://m1");
    let m_id = f.client.add_milestone(&ngo, &p_id, &10_000, &m_desc, &1, &verifiers);

    // Cannot release unapproved milestone
    assert!(f.client.try_release_milestone(&ngo, &p_id, &m_id).is_err());

    // Verifier approves
    let ev = String::from_str(&f.env, "ipfs://ev");
    f.client.approve_milestone(&v1, &p_id, &m_id, &ev);

    // Cannot release unfunded milestone
    assert!(f.client.try_release_milestone(&ngo, &p_id, &m_id).is_err());

    // Donor funds 6,000 (still less than 10,000 required)
    let donor = Address::generate(&f.env);
    f.mint(&donor, 20_000);
    f.client.fund_program(&donor, &p_id, &6_000);
    assert!(f.client.try_release_milestone(&ngo, &p_id, &m_id).is_err());

    // Donor funds additional 5,000 (total = 11,000)
    f.client.fund_program(&donor, &p_id, &5_000);

    // Non-NGO/non-admin caller cannot release
    let rando = Address::generate(&f.env);
    assert!(f.client.try_release_milestone(&rando, &p_id, &m_id).is_err());

    // NGO releases milestone
    f.client.release_milestone(&ngo, &p_id, &m_id);

    let prog = f.client.get_program(&p_id);
    assert_eq!(prog.total_funded, 11_000);
    assert_eq!(prog.total_released, 10_000);

    let m_rel = f.client.get_milestone(&p_id, &m_id);
    assert_eq!(m_rel.status, crate::types::MilestoneStatus::Released);

    // Cannot re-release already released milestone
    assert!(f.client.try_release_milestone(&ngo, &p_id, &m_id).is_err());
}

#[test]
fn test_vendor_registration_removal_and_categories() {
    let f = TestFixture::setup();
    let vendor = Address::generate(&f.env);

    let food = Symbol::new(&f.env, "FOOD");
    let med = Symbol::new(&f.env, "MEDICINE");
    let shelter = Symbol::new(&f.env, "SHELTER");

    let mut categories = Vec::new(&f.env);
    categories.push_back(food.clone());
    categories.push_back(med.clone());

    let meta = String::from_str(&f.env, "ipfs://vendor-profile");

    // Non-admin fails
    let rando = Address::generate(&f.env);
    assert!(f.client.try_register_vendor(&rando, &vendor, &categories, &meta).is_err());

    // Empty categories fail
    let empty_cats = Vec::new(&f.env);
    assert!(f.client.try_register_vendor(&f.admin, &vendor, &empty_cats, &meta).is_err());

    // Admin registers vendor
    f.client.register_vendor(&f.admin, &vendor, &categories, &meta);
    assert_eq!(f.client.get_vendor_count(), 1);

    let v = f.client.get_vendor(&vendor);
    assert_eq!(v.status, VendorStatus::Active);
    assert_eq!(v.total_redeemed, 0);
    assert_eq!(v.allowed_categories.len(), 2);

    assert_eq!(f.client.is_vendor_allowed(&vendor, &food), true);
    assert_eq!(f.client.is_vendor_allowed(&vendor, &med), true);
    assert_eq!(f.client.is_vendor_allowed(&vendor, &shelter), false);

    // Admin removes/suspends vendor
    f.client.remove_vendor(&f.admin, &vendor);
    let v_suspended = f.client.get_vendor(&vendor);
    assert_eq!(v_suspended.status, VendorStatus::Suspended);
    assert_eq!(f.client.is_vendor_allowed(&vendor, &food), false);

    // Admin reactivates vendor
    f.client.set_vendor_status(&f.admin, &vendor, &VendorStatus::Active);
    assert_eq!(f.client.get_vendor(&vendor).status, VendorStatus::Active);
    assert_eq!(f.client.is_vendor_allowed(&vendor, &food), true);
}

#[test]
fn test_issue_voucher_and_batch_issuance() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog-voucher");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    // Setup milestone and funding
    let v1 = Address::generate(&f.env);
    let mut verifiers = Vec::new(&f.env);
    verifiers.push_back(v1.clone());
    let m_id = f.client.add_milestone(
        &ngo,
        &p_id,
        &10_000,
        &String::from_str(&f.env, "m"),
        &1,
        &verifiers,
    );

    let donor = Address::generate(&f.env);
    f.mint(&donor, 20_000);
    f.client.fund_program(&donor, &p_id, &20_000);

    // Verifier approves & NGO releases
    f.client.approve_milestone(&v1, &p_id, &m_id, &String::from_str(&f.env, "e"));
    f.client.release_milestone(&ngo, &p_id, &m_id);
    // Program released = 10,000

    let ben1 = Address::generate(&f.env);
    let food = Symbol::new(&f.env, "FOOD");
    let expires = f.env.ledger().timestamp() + 86_400; // 1 day future

    // Issue voucher: 4,000
    let v_id1 = f.client.issue_voucher(&ngo, &p_id, &ben1, &4_000, &food, &expires);
    assert_eq!(v_id1, 1);
    assert_eq!(f.client.get_voucher_count(), 1);

    let v1_data = f.client.get_voucher(&v_id1);
    assert_eq!(v1_data.amount, 4_000);
    assert_eq!(v1_data.beneficiary, ben1);
    assert_eq!(v1_data.category, food);
    assert_eq!(v1_data.status, crate::types::VoucherStatus::Active);

    let prog_state = f.client.get_program(&p_id);
    assert_eq!(prog_state.total_allocated, 4_000);

    // Batch issuance: 3 vouchers of 2,000 each = 6,000 (remaining released: 6,000)
    let ben2 = Address::generate(&f.env);
    let ben3 = Address::generate(&f.env);
    let ben4 = Address::generate(&f.env);

    let mut batch = Vec::new(&f.env);
    batch.push_back(VoucherIssueRequest {
        beneficiary: ben2,
        amount: 2_000,
        category: food.clone(),
        expires_at: expires,
    });
    batch.push_back(VoucherIssueRequest {
        beneficiary: ben3,
        amount: 2_000,
        category: food.clone(),
        expires_at: expires,
    });
    batch.push_back(VoucherIssueRequest {
        beneficiary: ben4,
        amount: 2_000,
        category: food.clone(),
        expires_at: expires,
    });

    let issued = f.client.batch_issue_vouchers(&ngo, &p_id, &batch);
    assert_eq!(issued.len(), 3);
    assert_eq!(f.client.get_voucher_count(), 4);

    let prog_full = f.client.get_program(&p_id);
    assert_eq!(prog_full.total_allocated, 10_000);

    // Over-allocation fails (no more released funds available)
    let ben5 = Address::generate(&f.env);
    assert!(f.client.try_issue_voucher(&ngo, &p_id, &ben5, &500, &food, &expires).is_err());

    // Empty batch fails
    let empty_batch = Vec::new(&f.env);
    assert!(f.client.try_batch_issue_vouchers(&ngo, &p_id, &empty_batch).is_err());
}

#[test]
fn test_redeem_success_and_direct_vendor_payout() {
    let f = TestFixture::setup();
    let ngo = Address::generate(&f.env);
    let meta = String::from_str(&f.env, "ipfs://prog");
    let p_id = f.client.create_program(&ngo, &f.token_client.address, &meta);

    // Setup milestone (10,000) and funding (10,000)
    let v_verifier = Address::generate(&f.env);
    let mut verifiers = Vec::new(&f.env);
    verifiers.push_back(v_verifier.clone());
    let m_id = f.client.add_milestone(
        &ngo,
        &p_id,
        &10_000,
        &String::from_str(&f.env, "m"),
        &1,
        &verifiers,
    );

    let donor = Address::generate(&f.env);
    f.mint(&donor, 10_000);
    f.client.fund_program(&donor, &p_id, &10_000);

    f.client.approve_milestone(&v_verifier, &p_id, &m_id, &String::from_str(&f.env, "e"));
    f.client.release_milestone(&ngo, &p_id, &m_id);

    // Register 2 vendors: FoodVendor (FOOD) and MedVendor (MEDICINE)
    let food_vendor = Address::generate(&f.env);
    let med_vendor = Address::generate(&f.env);
    let food_cat = Symbol::new(&f.env, "FOOD");
    let med_cat = Symbol::new(&f.env, "MEDICINE");

    let mut food_cats = Vec::new(&f.env);
    food_cats.push_back(food_cat.clone());
    f.client.register_vendor(
        &f.admin,
        &food_vendor,
        &food_cats,
        &String::from_str(&f.env, "v1"),
    );

    let mut med_cats = Vec::new(&f.env);
    med_cats.push_back(med_cat.clone());
    f.client.register_vendor(
        &f.admin,
        &med_vendor,
        &med_cats,
        &String::from_str(&f.env, "v2"),
    );

    // Issue 3,000 FOOD voucher to beneficiary
    let beneficiary = Address::generate(&f.env);
    let expires = f.env.ledger().timestamp() + 86_400;
    let v_id = f.client.issue_voucher(&ngo, &p_id, &beneficiary, &3_000, &food_cat, &expires);

    // Fail: Trying to redeem FOOD voucher at MedVendor (category mismatch)
    assert!(f.client.try_redeem(&beneficiary, &v_id, &med_vendor).is_err());

    // Fail: Unauthorized caller (not beneficiary)
    let imposter = Address::generate(&f.env);
    assert!(f.client.try_redeem(&imposter, &v_id, &food_vendor).is_err());

    // Success: Beneficiary redeems at FoodVendor
    assert_eq!(f.token_client.balance(&food_vendor), 0);
    assert_eq!(f.token_client.balance(&f.contract_id), 10_000);

    f.client.redeem(&beneficiary, &v_id, &food_vendor);

    // Direct payout verified: FoodVendor receives 3,000, contract balance is 7,000
    assert_eq!(f.token_client.balance(&food_vendor), 3_000);
    assert_eq!(f.token_client.balance(&f.contract_id), 7_000);

    let voucher_post = f.client.get_voucher(&v_id);
    assert_eq!(voucher_post.status, crate::types::VoucherStatus::Redeemed);

    let vendor_post = f.client.get_vendor(&food_vendor);
    assert_eq!(vendor_post.total_redeemed, 3_000);

    // Fail: Cannot double-redeem
    assert!(f.client.try_redeem(&beneficiary, &v_id, &food_vendor).is_err());
}








