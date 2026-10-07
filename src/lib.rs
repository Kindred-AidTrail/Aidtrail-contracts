#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use errors::ContractError;
use events::Events;
use storage::Storage;
use types::{DonorContribution, Milestone, MilestoneStatus, Program, ProgramStatus};

use soroban_sdk::{contract, contractimpl, token, Address, Env, String, Vec};

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
