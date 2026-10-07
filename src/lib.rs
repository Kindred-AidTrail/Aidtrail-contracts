#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use errors::ContractError;
use events::Events;
use storage::Storage;
use types::{Program, ProgramStatus};

use soroban_sdk::{contract, contractimpl, Address, Env, String};

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
