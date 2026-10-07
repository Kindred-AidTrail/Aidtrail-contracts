#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AidtrailContract;

#[contractimpl]
impl AidtrailContract {
    pub fn ping(_env: Env) -> bool {
        true
    }
}
