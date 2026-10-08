use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

pub struct Events;

impl Events {
    pub fn initialized(env: &Env, admin: &Address) {
        let topics = (symbol_short!("contract"), symbol_short!("init"));
        env.events().publish(topics, admin.clone());
    }

    pub fn paused(env: &Env, paused: bool) {
        let topics = (symbol_short!("contract"), symbol_short!("paused"));
        env.events().publish(topics, paused);
    }

    pub fn program_created(
        env: &Env,
        program_id: u64,
        ngo: &Address,
        token: &Address,
        metadata_uri: &String,
    ) {
        let topics = (symbol_short!("program"), symbol_short!("created"));
        let data = (program_id, ngo.clone(), token.clone(), metadata_uri.clone());
        env.events().publish(topics, data);
    }

    pub fn program_funded(env: &Env, program_id: u64, donor: &Address, amount: i128) {
        let topics = (symbol_short!("program"), symbol_short!("funded"));
        let data = (program_id, donor.clone(), amount);
        env.events().publish(topics, data);
    }

    pub fn program_cancelled(env: &Env, program_id: u64, refundable_pool: i128) {
        let topics = (symbol_short!("program"), symbol_short!("cancel"));
        let data = (program_id, refundable_pool);
        env.events().publish(topics, data);
    }

    pub fn donor_refunded(env: &Env, program_id: u64, donor: &Address, amount: i128) {
        let topics = (symbol_short!("program"), symbol_short!("refund"));
        let data = (program_id, donor.clone(), amount);
        env.events().publish(topics, data);
    }

    pub fn milestone_added(
        env: &Env,
        program_id: u64,
        milestone_id: u32,
        amount: i128,
        required_approvals: u32,
    ) {
        let topics = (symbol_short!("milestn"), symbol_short!("added"));
        let data = (program_id, milestone_id, amount, required_approvals);
        env.events().publish(topics, data);
    }

    pub fn milestone_approved(
        env: &Env,
        program_id: u64,
        milestone_id: u32,
        verifier: &Address,
        approvals_count: u32,
    ) {
        let topics = (symbol_short!("milestn"), symbol_short!("apprvd"));
        let data = (program_id, milestone_id, verifier.clone(), approvals_count);
        env.events().publish(topics, data);
    }

    pub fn milestone_verifiers_updated(
        env: &Env,
        program_id: u64,
        milestone_id: u32,
        required_approvals: u32,
    ) {
        let topics = (symbol_short!("milestn"), symbol_short!("ver_upd"));
        let data = (program_id, milestone_id, required_approvals);
        env.events().publish(topics, data);
    }

    pub fn milestone_released(env: &Env, program_id: u64, milestone_id: u32, amount: i128) {
        let topics = (symbol_short!("milestn"), symbol_short!("release"));
        let data = (program_id, milestone_id, amount);
        env.events().publish(topics, data);
    }

    pub fn vendor_registered(env: &Env, vendor: &Address, allowed_categories: &Vec<Symbol>) {
        let topics = (symbol_short!("vendor"), symbol_short!("reg"));
        let data = (vendor.clone(), allowed_categories.clone());
        env.events().publish(topics, data);
    }

    pub fn vendor_removed(env: &Env, vendor: &Address) {
        let topics = (symbol_short!("vendor"), symbol_short!("rem"));
        env.events().publish(topics, vendor.clone());
    }

    pub fn voucher_issued(
        env: &Env,
        voucher_id: u64,
        program_id: u64,
        beneficiary: &Address,
        amount: i128,
        category: &Symbol,
        expires_at: u64,
    ) {
        let topics = (symbol_short!("voucher"), symbol_short!("issued"));
        let data = (
            voucher_id,
            program_id,
            beneficiary.clone(),
            amount,
            category.clone(),
            expires_at,
        );
        env.events().publish(topics, data);
    }

    pub fn voucher_redeemed(
        env: &Env,
        voucher_id: u64,
        beneficiary: &Address,
        vendor: &Address,
        amount: i128,
    ) {
        let topics = (symbol_short!("voucher"), symbol_short!("redeem"));
        let data = (voucher_id, beneficiary.clone(), vendor.clone(), amount);
        env.events().publish(topics, data);
    }

    pub fn voucher_reclaimed(env: &Env, voucher_id: u64, program_id: u64, amount: i128) {
        let topics = (symbol_short!("voucher"), symbol_short!("reclaim"));
        let data = (voucher_id, program_id, amount);
        env.events().publish(topics, data);
    }
}
