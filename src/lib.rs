#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

#[cfg(test)]
mod test;

use errors::ContractError;
use events::Events;
use storage::Storage;
use types::{
    ContractStats, DonorContribution, Milestone, MilestoneStatus, Program, ProgramStatus, Vendor,
    VendorStatus, Voucher, VoucherIssueRequest, VoucherStatus,
};

use soroban_sdk::{contract, contractimpl, token, Address, Env, String, Symbol, Vec};

#[contract]
pub struct AidtrailContract;

#[contractimpl]
impl AidtrailContract {
    /// Initialize the AidTrail contract with an admin and optional emergency admin.
    pub fn initialize(
        env: Env,
        admin: Address,
        emergency_admin: Option<Address>,
    ) -> Result<(), ContractError> {
        if Storage::get_admin(&env).is_some() {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();

        Storage::set_admin(&env, &admin);
        if let Some(ref em_admin) = emergency_admin {
            Storage::set_emergency_admin(&env, em_admin);
        }
        Storage::set_paused(&env, false);

        Events::initialized(&env, &admin);
        Ok(())
    }

    /// Toggle emergency circuit breaker (pause/unpause).
    pub fn set_paused(env: Env, caller: Address, paused: bool) -> Result<(), ContractError> {
        caller.require_auth();

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        let emergency_admin = Storage::get_emergency_admin(&env);

        let is_authorized = caller == admin
            || emergency_admin.map_or(false, |ea| ea == caller);

        if !is_authorized {
            return Err(ContractError::Unauthorized);
        }

        Storage::set_paused(&env, paused);
        Events::paused(&env, paused);
        Ok(())
    }

    /// Transfer contract administrative ownership.
    pub fn set_admin(
        env: Env,
        current_admin: Address,
        new_admin: Address,
    ) -> Result<(), ContractError> {
        current_admin.require_auth();
        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if current_admin != admin {
            return Err(ContractError::Unauthorized);
        }

        Storage::set_admin(&env, &new_admin);
        Events::initialized(&env, &new_admin);
        Ok(())
    }

    /// Set or update the emergency admin address.
    pub fn set_emergency_admin(
        env: Env,
        admin: Address,
        emergency_admin: Option<Address>,
    ) -> Result<(), ContractError> {
        admin.require_auth();
        let current_admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if admin != current_admin {
            return Err(ContractError::Unauthorized);
        }

        if let Some(ref em_admin) = emergency_admin {
            Storage::set_emergency_admin(&env, em_admin);
        }
        Ok(())
    }

    /// Create a new aid program with specified payment token and metadata.
    pub fn create_program(
        env: Env,
        ngo: Address,
        token: Address,
        metadata_uri: String,
    ) -> Result<u64, ContractError> {
        if Storage::get_admin(&env).is_none() {
            return Err(ContractError::NotInitialized);
        }
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        ngo.require_auth();

        let program_id = Storage::increment_program_count(&env);
        let program = Program {
            id: program_id,
            ngo: ngo.clone(),
            token: token.clone(),
            metadata_uri: metadata_uri.clone(),
            status: ProgramStatus::Active,
            total_funded: 0,
            total_released: 0,
            total_allocated: 0,
            refundable_pool: 0,
            created_at: env.ledger().timestamp(),
        };

        Storage::set_program(&env, &program);
        Events::program_created(&env, program_id, &ngo, &token, &metadata_uri);

        Ok(program_id)
    }

    /// Add a milestone to an aid program with M-of-N verifier approval requirements.
    pub fn add_milestone(
        env: Env,
        caller: Address,
        program_id: u64,
        amount: i128,
        description_uri: String,
        required_approvals: u32,
        verifiers: Vec<Address>,
    ) -> Result<u32, ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let verifier_count = verifiers.len();
        if required_approvals == 0 || required_approvals > verifier_count {
            return Err(ContractError::InvalidMilestoneConfig);
        }

        // Ensure verifiers list has no duplicate addresses
        for i in 0..verifier_count {
            let v_i = verifiers.get(i).unwrap();
            for j in (i + 1)..verifier_count {
                let v_j = verifiers.get(j).unwrap();
                if v_i == v_j {
                    return Err(ContractError::InvalidMilestoneConfig);
                }
            }
        }

        let milestone_id = Storage::increment_milestone_count(&env, program_id);
        let milestone = Milestone {
            id: milestone_id,
            program_id,
            amount,
            description_uri,
            status: MilestoneStatus::Pending,
            required_approvals,
            verifiers,
            approvals: Vec::new(&env),
        };

        Storage::set_milestone(&env, &milestone);
        Events::milestone_added(&env, program_id, milestone_id, amount, required_approvals);

        Ok(milestone_id)
    }

    /// Donors fund an aid program by transferring tokens into contract custody.
    pub fn fund_program(
        env: Env,
        donor: Address,
        program_id: u64,
        amount: i128,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        donor.require_auth();

        let mut program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        // Accounting update with overflow guard (Checks-Effects-Interactions)
        let new_funded = program
            .total_funded
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        program.total_funded = new_funded;
        Storage::set_program(&env, &program);

        // Update donor contribution tracking
        let mut contribution = Storage::get_donor_contribution(&env, program_id, &donor)
            .unwrap_or(DonorContribution {
                donor: donor.clone(),
                program_id,
                amount: 0,
                refunded: false,
            });
        let new_contrib = contribution
            .amount
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        contribution.amount = new_contrib;
        Storage::set_donor_contribution(&env, &contribution);

        Events::program_funded(&env, program_id, &donor, amount);

        // Interaction: pull tokens from donor to contract
        let token_client = token::Client::new(&env, &program.token);
        token_client.transfer(&donor, &env.current_contract_address(), &amount);

        Ok(())
    }

    /// Submit independent verifier approval for a milestone with evidence.
    pub fn approve_milestone(
        env: Env,
        verifier: Address,
        program_id: u64,
        milestone_id: u32,
        _evidence_uri: String,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        verifier.require_auth();

        let program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let mut milestone = Storage::get_milestone(&env, program_id, milestone_id)
            .ok_or(ContractError::MilestoneNotFound)?;

        if milestone.status == MilestoneStatus::Released {
            return Err(ContractError::MilestoneAlreadyReleased);
        }

        // Validate that verifier is an authorized verifier for this milestone
        let mut is_authorized = false;
        for v in milestone.verifiers.iter() {
            if v == verifier {
                is_authorized = true;
                break;
            }
        }
        if !is_authorized {
            return Err(ContractError::VerifierNotAuthorized);
        }

        // Ensure no duplicate approval from the same verifier
        for app in milestone.approvals.iter() {
            if app == verifier {
                return Err(ContractError::DuplicateVerifierApproval);
            }
        }

        milestone.approvals.push_back(verifier.clone());
        let approvals_count = milestone.approvals.len();

        // If threshold reached, transition status to Approved
        if approvals_count >= milestone.required_approvals {
            milestone.status = MilestoneStatus::Approved;
        }

        Storage::set_milestone(&env, &milestone);
        Events::milestone_approved(&env, program_id, milestone_id, &verifier, approvals_count);

        Ok(())
    }

    /// Release an approved milestone, validating funded >= released accounting invariant.
    pub fn release_milestone(
        env: Env,
        caller: Address,
        program_id: u64,
        milestone_id: u32,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let mut program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        let mut milestone = Storage::get_milestone(&env, program_id, milestone_id)
            .ok_or(ContractError::MilestoneNotFound)?;

        if milestone.status == MilestoneStatus::Released {
            return Err(ContractError::MilestoneAlreadyReleased);
        }
        if milestone.status != MilestoneStatus::Approved {
            return Err(ContractError::MilestoneNotApproved);
        }

        // Strict invariant: total_funded >= total_released + milestone.amount
        let new_released = program
            .total_released
            .checked_add(milestone.amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if program.total_funded < new_released {
            return Err(ContractError::InsufficientProgramFunds);
        }

        program.total_released = new_released;
        milestone.status = MilestoneStatus::Released;

        Storage::set_program(&env, &program);
        Storage::set_milestone(&env, &milestone);

        Events::milestone_released(&env, program_id, milestone_id, milestone.amount);

        Ok(())
    }

    /// Register a vendor with allowed redemption categories and metadata.
    pub fn register_vendor(
        env: Env,
        caller: Address,
        vendor_address: Address,
        allowed_categories: Vec<Symbol>,
        metadata_uri: String,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        if allowed_categories.is_empty() {
            return Err(ContractError::VendorCategoryNotAllowed);
        }

        let existing = Storage::get_vendor(&env, &vendor_address);
        if existing.is_none() {
            let count = Storage::get_vendor_count(&env) + 1;
            Storage::set_vendor_count(&env, count);
        }

        let total_redeemed = existing.map_or(0, |v| v.total_redeemed);
        let vendor = Vendor {
            address: vendor_address.clone(),
            status: VendorStatus::Active,
            allowed_categories: allowed_categories.clone(),
            metadata_uri,
            total_redeemed,
        };

        Storage::set_vendor(&env, &vendor);
        Events::vendor_registered(&env, &vendor_address, &allowed_categories);

        Ok(())
    }

    /// Suspend or remove a vendor from participating in voucher redemptions.
    pub fn remove_vendor(
        env: Env,
        caller: Address,
        vendor_address: Address,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        let mut vendor = Storage::get_vendor(&env, &vendor_address)
            .ok_or(ContractError::VendorNotFound)?;

        vendor.status = VendorStatus::Suspended;
        Storage::set_vendor(&env, &vendor);
        Events::vendor_removed(&env, &vendor_address);

        Ok(())
    }

    /// Update status of a vendor (Active or Suspended).
    pub fn set_vendor_status(
        env: Env,
        caller: Address,
        vendor_address: Address,
        status: VendorStatus,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        let mut vendor = Storage::get_vendor(&env, &vendor_address)
            .ok_or(ContractError::VendorNotFound)?;

        vendor.status = status;
        Storage::set_vendor(&env, &vendor);

        if status == VendorStatus::Suspended {
            Events::vendor_removed(&env, &vendor_address);
        } else {
            Events::vendor_registered(&env, &vendor_address, &vendor.allowed_categories);
        }

        Ok(())
    }

    /// Issue a voucher to a beneficiary against unlocked milestone funds.
    pub fn issue_voucher(
        env: Env,
        caller: Address,
        program_id: u64,
        beneficiary: Address,
        amount: i128,
        category: Symbol,
        expires_at: u64,
    ) -> Result<u64, ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let mut program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        if expires_at <= env.ledger().timestamp() {
            return Err(ContractError::InvalidExpiration);
        }

        // Invariant: total_released >= total_allocated + amount
        let new_allocated = program
            .total_allocated
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if program.total_released < new_allocated {
            return Err(ContractError::InsufficientProgramFunds);
        }

        program.total_allocated = new_allocated;
        Storage::set_program(&env, &program);

        let voucher_id = Storage::increment_voucher_count(&env);
        let voucher = Voucher {
            id: voucher_id,
            program_id,
            beneficiary: beneficiary.clone(),
            amount,
            category: category.clone(),
            status: VoucherStatus::Active,
            expires_at,
            created_at: env.ledger().timestamp(),
        };

        Storage::set_voucher(&env, &voucher);
        Events::voucher_issued(
            &env,
            voucher_id,
            program_id,
            &beneficiary,
            amount,
            &category,
            expires_at,
        );

        Ok(voucher_id)
    }

    /// Batch issue vouchers to beneficiaries in a single transaction.
    pub fn batch_issue_vouchers(
        env: Env,
        caller: Address,
        program_id: u64,
        vouchers: Vec<VoucherIssueRequest>,
    ) -> Result<Vec<u64>, ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let mut program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;
        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        let count = vouchers.len();
        if count == 0 {
            return Err(ContractError::EmptyBatch);
        }
        if count > 50 {
            return Err(ContractError::BatchSizeExceeded);
        }

        let current_time = env.ledger().timestamp();
        let mut total_batch_amount: i128 = 0;

        for req in vouchers.iter() {
            if req.amount <= 0 {
                return Err(ContractError::InvalidAmount);
            }
            if req.expires_at <= current_time {
                return Err(ContractError::InvalidExpiration);
            }
            total_batch_amount = total_batch_amount
                .checked_add(req.amount)
                .ok_or(ContractError::ArithmeticOverflow)?;
        }

        // Invariant: total_released >= total_allocated + total_batch_amount
        let new_allocated = program
            .total_allocated
            .checked_add(total_batch_amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if program.total_released < new_allocated {
            return Err(ContractError::InsufficientProgramFunds);
        }

        program.total_allocated = new_allocated;
        Storage::set_program(&env, &program);

        let mut issued_ids: Vec<u64> = Vec::new(&env);
        for req in vouchers.iter() {
            let voucher_id = Storage::increment_voucher_count(&env);
            let voucher = Voucher {
                id: voucher_id,
                program_id,
                beneficiary: req.beneficiary.clone(),
                amount: req.amount,
                category: req.category.clone(),
                status: VoucherStatus::Active,
                expires_at: req.expires_at,
                created_at: current_time,
            };

            Storage::set_voucher(&env, &voucher);
            Events::voucher_issued(
                &env,
                voucher_id,
                program_id,
                &req.beneficiary,
                req.amount,
                &req.category,
                req.expires_at,
            );
            issued_ids.push_back(voucher_id);
        }

        Ok(issued_ids)
    }

    /// Redeem an active voucher at an approved vendor matching the voucher's category.
    pub fn redeem(
        env: Env,
        beneficiary: Address,
        voucher_id: u64,
        vendor_address: Address,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        beneficiary.require_auth();

        let mut voucher = Storage::get_voucher(&env, voucher_id)
            .ok_or(ContractError::VoucherNotFound)?;

        if voucher.beneficiary != beneficiary {
            return Err(ContractError::Unauthorized);
        }
        if voucher.status != VoucherStatus::Active {
            return Err(ContractError::VoucherNotActive);
        }
        if env.ledger().timestamp() >= voucher.expires_at {
            return Err(ContractError::VoucherExpired);
        }

        let mut vendor = Storage::get_vendor(&env, &vendor_address)
            .ok_or(ContractError::VendorNotFound)?;

        if vendor.status != VendorStatus::Active {
            return Err(ContractError::VendorNotActive);
        }

        // Enforce per-voucher category permission on the vendor
        let mut category_allowed = false;
        for cat in vendor.allowed_categories.iter() {
            if cat == voucher.category {
                category_allowed = true;
                break;
            }
        }
        if !category_allowed {
            return Err(ContractError::VendorCategoryNotAllowed);
        }

        let program = Storage::get_program(&env, voucher.program_id)
            .ok_or(ContractError::ProgramNotFound)?;

        // Checks-Effects-Interactions: state updates occur BEFORE token transfer
        voucher.status = VoucherStatus::Redeemed;
        vendor.total_redeemed = vendor
            .total_redeemed
            .checked_add(voucher.amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        Storage::set_voucher(&env, &voucher);
        Storage::set_vendor(&env, &vendor);

        Events::voucher_redeemed(&env, voucher_id, &beneficiary, &vendor_address, voucher.amount);

        // Direct payout to the vendor from contract balance
        let token_client = token::Client::new(&env, &program.token);
        token_client.transfer(
            &env.current_contract_address(),
            &vendor_address,
            &voucher.amount,
        );

        Ok(())
    }

    /// Reclaim funds from an expired, unredeemed voucher back to the program allocation pool.
    pub fn reclaim_expired(
        env: Env,
        caller: Address,
        voucher_id: u64,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let mut voucher = Storage::get_voucher(&env, voucher_id)
            .ok_or(ContractError::VoucherNotFound)?;

        if voucher.status != VoucherStatus::Active {
            return Err(ContractError::VoucherNotActive);
        }
        if env.ledger().timestamp() < voucher.expires_at {
            return Err(ContractError::VoucherNotExpired);
        }

        let mut program = Storage::get_program(&env, voucher.program_id)
            .ok_or(ContractError::ProgramNotFound)?;

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        // Return amount to unallocated pool: decrement total_allocated
        program.total_allocated = program
            .total_allocated
            .checked_sub(voucher.amount)
            .ok_or(ContractError::ArithmeticOverflow)?;

        voucher.status = VoucherStatus::Reclaimed;

        Storage::set_voucher(&env, &voucher);
        Storage::set_program(&env, &program);

        Events::voucher_reclaimed(&env, voucher_id, voucher.program_id, voucher.amount);

        Ok(())
    }

    /// Cancel an active aid program, freezing unreleased funds into a refundable donor pool.
    pub fn cancel_program(
        env: Env,
        caller: Address,
        program_id: u64,
    ) -> Result<(), ContractError> {
        if Storage::is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }
        caller.require_auth();

        let mut program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;

        if program.status != ProgramStatus::Active {
            return Err(ContractError::ProgramNotActive);
        }

        let admin = Storage::get_admin(&env).ok_or(ContractError::NotInitialized)?;
        if caller != program.ngo && caller != admin {
            return Err(ContractError::Unauthorized);
        }

        // Calculate unreleased funds available for donor refund
        let refundable_pool = if program.total_funded > program.total_released {
            program
                .total_funded
                .checked_sub(program.total_released)
                .ok_or(ContractError::ArithmeticOverflow)?
        } else {
            0
        };

        program.status = ProgramStatus::Cancelled;
        program.refundable_pool = refundable_pool;
        Storage::set_program(&env, &program);

        Events::program_cancelled(&env, program_id, refundable_pool);

        Ok(())
    }

    /// Claim proportional refund of unreleased program capital as a donor.
    pub fn claim_donor_refund(
        env: Env,
        donor: Address,
        program_id: u64,
    ) -> Result<i128, ContractError> {
        donor.require_auth();

        let program = Storage::get_program(&env, program_id)
            .ok_or(ContractError::ProgramNotFound)?;

        if program.status != ProgramStatus::Cancelled {
            return Err(ContractError::ProgramNotActive);
        }

        if program.refundable_pool <= 0 || program.total_funded <= 0 {
            return Err(ContractError::NothingToRefund);
        }

        let mut contribution = Storage::get_donor_contribution(&env, program_id, &donor)
            .ok_or(ContractError::DonorContributionNotFound)?;

        if contribution.refunded || contribution.amount <= 0 {
            return Err(ContractError::AlreadyRefunded);
        }

        // Proportional refund: (donor_contrib * refundable_pool) / total_funded
        let refund_amount = contribution
            .amount
            .checked_mul(program.refundable_pool)
            .ok_or(ContractError::ArithmeticOverflow)?
            .checked_div(program.total_funded)
            .ok_or(ContractError::ArithmeticOverflow)?;

        if refund_amount <= 0 {
            return Err(ContractError::NothingToRefund);
        }

        // Checks-Effects-Interactions: record refund before token transfer
        contribution.refunded = true;
        Storage::set_donor_contribution(&env, &contribution);

        Events::donor_refunded(&env, program_id, &donor, refund_amount);

        let token_client = token::Client::new(&env, &program.token);
        token_client.transfer(
            &env.current_contract_address(),
            &donor,
            &refund_amount,
        );

        Ok(refund_amount)
    }

    /// Query global platform statistics for dashboard and audit explorer.
    pub fn get_contract_stats(env: Env) -> ContractStats {
        let total_programs = Storage::get_program_count(&env);
        let total_vouchers = Storage::get_voucher_count(&env);
        let total_vendors = Storage::get_vendor_count(&env);

        let mut total_funded_volume: i128 = 0;
        let mut total_released_volume: i128 = 0;

        for id in 1..=total_programs {
            if let Some(prog) = Storage::get_program(&env, id) {
                total_funded_volume = total_funded_volume.saturating_add(prog.total_funded);
                total_released_volume = total_released_volume.saturating_add(prog.total_released);
            }
        }

        let mut total_redeemed_volume: i128 = 0;
        for v_id in 1..=total_vouchers {
            if let Some(voucher) = Storage::get_voucher(&env, v_id) {
                if voucher.status == VoucherStatus::Redeemed {
                    total_redeemed_volume = total_redeemed_volume.saturating_add(voucher.amount);
                }
            }
        }

        ContractStats {
            total_programs,
            total_vouchers,
            total_vendors,
            total_funded_volume,
            total_released_volume,
            total_redeemed_volume,
        }
    }

    /// Retrieve paginated list of programs.
    pub fn get_all_programs(env: Env, start_id: u64, limit: u32) -> Vec<Program> {
        let mut programs = Vec::new(&env);
        let total = Storage::get_program_count(&env);
        if start_id == 0 || start_id > total {
            return programs;
        }

        let end = (start_id + (limit as u64) - 1).min(total);
        for id in start_id..=end {
            if let Some(prog) = Storage::get_program(&env, id) {
                programs.push_back(prog);
            }
        }
        programs
    }

    /// Retrieve all milestones belonging to a program.
    pub fn get_program_milestones(env: Env, program_id: u64) -> Vec<Milestone> {
        let mut milestones = Vec::new(&env);
        let count = Storage::get_milestone_count(&env, program_id);
        for m_id in 1..=count {
            if let Some(m) = Storage::get_milestone(&env, program_id, m_id) {
                milestones.push_back(m);
            }
        }
        milestones
    }

    /// Check if a milestone has reached its required approval threshold.
    pub fn is_milestone_approved(env: Env, program_id: u64, milestone_id: u32) -> bool {
        Storage::get_milestone(&env, program_id, milestone_id).map_or(false, |m| {
            m.status == MilestoneStatus::Approved || m.status == MilestoneStatus::Released
        })
    }

    /// Check if a vendor is active and permitted to redeem vouchers of a given category.
    pub fn is_vendor_allowed(env: Env, vendor: Address, category: Symbol) -> bool {
        Storage::get_vendor(&env, &vendor).map_or(false, |v| {
            if v.status != VendorStatus::Active {
                return false;
            }
            for cat in v.allowed_categories.iter() {
                if cat == category {
                    return true;
                }
            }
            false
        })
    }

    /// Query a voucher by its unique ID.
    pub fn get_voucher(env: Env, voucher_id: u64) -> Result<Voucher, ContractError> {
        Storage::get_voucher(&env, voucher_id).ok_or(ContractError::VoucherNotFound)
    }

    /// Query total number of vouchers created.
    pub fn get_voucher_count(env: Env) -> u64 {
        Storage::get_voucher_count(&env)
    }

    /// Query vendor profile by vendor address.
    pub fn get_vendor(env: Env, vendor: Address) -> Result<Vendor, ContractError> {
        Storage::get_vendor(&env, &vendor).ok_or(ContractError::VendorNotFound)
    }

    /// Query total number of registered vendors.
    pub fn get_vendor_count(env: Env) -> u32 {
        Storage::get_vendor_count(&env)
    }

    /// Query contribution record for a specific donor and program.
    pub fn get_donor_contribution(
        env: Env,
        program_id: u64,
        donor: Address,
    ) -> Option<DonorContribution> {
        Storage::get_donor_contribution(&env, program_id, &donor)
    }

    /// Query a milestone by program ID and milestone ID.
    pub fn get_milestone(
        env: Env,
        program_id: u64,
        milestone_id: u32,
    ) -> Result<Milestone, ContractError> {
        Storage::get_milestone(&env, program_id, milestone_id)
            .ok_or(ContractError::MilestoneNotFound)
    }

    /// Query the total number of milestones for a given program.
    pub fn get_milestone_count(env: Env, program_id: u64) -> u32 {
        Storage::get_milestone_count(&env, program_id)
    }

    /// Query a program by its ID.
    pub fn get_program(env: Env, program_id: u64) -> Result<Program, ContractError> {
        Storage::get_program(&env, program_id).ok_or(ContractError::ProgramNotFound)
    }

    /// Query total number of created programs.
    pub fn get_program_count(env: Env) -> u64 {
        Storage::get_program_count(&env)
    }

    /// Query whether contract operations are currently paused.
    pub fn is_paused(env: Env) -> bool {
        Storage::is_paused(&env)
    }

    /// Query current contract admin address.
    pub fn get_admin(env: Env) -> Option<Address> {
        Storage::get_admin(&env)
    }

    /// Query current contract emergency admin address.
    pub fn get_emergency_admin(env: Env) -> Option<Address> {
        Storage::get_emergency_admin(&env)
    }
}
