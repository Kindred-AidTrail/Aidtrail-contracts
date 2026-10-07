#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AidtrailContract;

#[contractimpl]
impl AidtrailContract {
    pub fn ping(_env: Env) -> bool {
        true
    }
}
