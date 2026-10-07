use crate::types::{DonorContribution, Milestone, Program, Vendor, Voucher};
use soroban_sdk::{contracttype, Address, Env};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 172_800;
pub const INSTANCE_BUMP_AMOUNT: u32 = 518_400;

pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 172_800;
pub const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Admin,
    EmergencyAdmin,
    Paused,
    ProgramCount,
    VoucherCount,
    VendorCount,
    Program(u64),
    Milestone(u64, u32),
    MilestoneCount(u64),
    DonorContribution(u64, Address),
    Vendor(Address),
    Voucher(u64),
}

pub struct Storage;

impl Storage {
    pub fn extend_instance_ttl(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    pub fn extend_persistent_ttl(env: &Env, key: &DataKey) {
        env.storage().persistent().extend_ttl(
            key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }

    // Admin
    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&DataKey::Admin, admin);
        Self::extend_instance_ttl(env);
    }

    // Emergency Admin
    pub fn get_emergency_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::EmergencyAdmin)
    }

    pub fn set_emergency_admin(env: &Env, emergency_admin: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::EmergencyAdmin, emergency_admin);
        Self::extend_instance_ttl(env);
    }

    // Paused
    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }

    pub fn set_paused(env: &Env, paused: bool) {
        env.storage().instance().set(&DataKey::Paused, &paused);
        Self::extend_instance_ttl(env);
    }

    // Counters
    pub fn get_program_count(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ProgramCount)
            .unwrap_or(0)
    }

    pub fn increment_program_count(env: &Env) -> u64 {
        let count = Self::get_program_count(env) + 1;
        env.storage().instance().set(&DataKey::ProgramCount, &count);
        Self::extend_instance_ttl(env);
        count
    }

    pub fn get_voucher_count(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::VoucherCount)
            .unwrap_or(0)
    }

    pub fn increment_voucher_count(env: &Env) -> u64 {
        let count = Self::get_voucher_count(env) + 1;
        env.storage().instance().set(&DataKey::VoucherCount, &count);
        Self::extend_instance_ttl(env);
        count
    }

    pub fn get_vendor_count(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::VendorCount)
            .unwrap_or(0)
    }

    pub fn set_vendor_count(env: &Env, count: u32) {
        env.storage().instance().set(&DataKey::VendorCount, &count);
        Self::extend_instance_ttl(env);
    }

    // Program
    pub fn get_program(env: &Env, program_id: u64) -> Option<Program> {
        let key = DataKey::Program(program_id);
        let prog = env.storage().persistent().get(&key);
        if prog.is_some() {
            Self::extend_persistent_ttl(env, &key);
        }
        prog
    }

    pub fn set_program(env: &Env, program: &Program) {
        let key = DataKey::Program(program.id);
        env.storage().persistent().set(&key, program);
        Self::extend_persistent_ttl(env, &key);
    }

    // Milestones
    pub fn get_milestone_count(env: &Env, program_id: u64) -> u32 {
        let key = DataKey::MilestoneCount(program_id);
        let count = env.storage().persistent().get(&key).unwrap_or(0);
        if count > 0 {
            Self::extend_persistent_ttl(env, &key);
        }
        count
    }

    pub fn increment_milestone_count(env: &Env, program_id: u64) -> u32 {
        let key = DataKey::MilestoneCount(program_id);
        let count = Self::get_milestone_count(env, program_id) + 1;
        env.storage().persistent().set(&key, &count);
        Self::extend_persistent_ttl(env, &key);
        count
    }

    pub fn get_milestone(env: &Env, program_id: u64, milestone_id: u32) -> Option<Milestone> {
        let key = DataKey::Milestone(program_id, milestone_id);
        let m = env.storage().persistent().get(&key);
        if m.is_some() {
            Self::extend_persistent_ttl(env, &key);
        }
        m
    }

    pub fn set_milestone(env: &Env, milestone: &Milestone) {
        let key = DataKey::Milestone(milestone.program_id, milestone.id);
        env.storage().persistent().set(&key, milestone);
        Self::extend_persistent_ttl(env, &key);
    }

    // Donor Contribution
    pub fn get_donor_contribution(
        env: &Env,
        program_id: u64,
        donor: &Address,
    ) -> Option<DonorContribution> {
        let key = DataKey::DonorContribution(program_id, donor.clone());
        let c = env.storage().persistent().get(&key);
        if c.is_some() {
            Self::extend_persistent_ttl(env, &key);
        }
        c
    }

    pub fn set_donor_contribution(env: &Env, contribution: &DonorContribution) {
        let key = DataKey::DonorContribution(contribution.program_id, contribution.donor.clone());
        env.storage().persistent().set(&key, contribution);
        Self::extend_persistent_ttl(env, &key);
    }

    // Vendor
    pub fn get_vendor(env: &Env, vendor: &Address) -> Option<Vendor> {
        let key = DataKey::Vendor(vendor.clone());
        let v = env.storage().persistent().get(&key);
        if v.is_some() {
            Self::extend_persistent_ttl(env, &key);
        }
        v
    }

    pub fn set_vendor(env: &Env, vendor: &Vendor) {
        let key = DataKey::Vendor(vendor.address.clone());
        env.storage().persistent().set(&key, vendor);
        Self::extend_persistent_ttl(env, &key);
    }

    // Voucher
    pub fn get_voucher(env: &Env, voucher_id: u64) -> Option<Voucher> {
        let key = DataKey::Voucher(voucher_id);
        let v = env.storage().persistent().get(&key);
        if v.is_some() {
            Self::extend_persistent_ttl(env, &key);
        }
        v
    }

    pub fn set_voucher(env: &Env, voucher: &Voucher) {
        let key = DataKey::Voucher(voucher.id);
        env.storage().persistent().set(&key, voucher);
        Self::extend_persistent_ttl(env, &key);
    }
}
