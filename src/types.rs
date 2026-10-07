use soroban_sdk::{contracttype, Address, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgramStatus {
    Active = 0,
    Completed = 1,
    Cancelled = 2,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MilestoneStatus {
    Pending = 0,
    Approved = 1,
    Released = 2,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoucherStatus {
    Active = 0,
    Redeemed = 1,
    Reclaimed = 2,
    Cancelled = 3,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VendorStatus {
    Active = 0,
    Suspended = 1,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub id: u64,
    pub ngo: Address,
    pub token: Address,
    pub metadata_uri: String,
    pub status: ProgramStatus,
    pub total_funded: i128,
    pub total_released: i128,
    pub total_allocated: i128,
    pub refundable_pool: i128,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Milestone {
    pub id: u32,
    pub program_id: u64,
    pub amount: i128,
    pub description_uri: String,
    pub status: MilestoneStatus,
    pub required_approvals: u32,
    pub verifiers: Vec<Address>,
    pub approvals: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Voucher {
    pub id: u64,
    pub program_id: u64,
    pub beneficiary: Address,
    pub amount: i128,
    pub category: Symbol,
    pub status: VoucherStatus,
    pub expires_at: u64,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Vendor {
    pub address: Address,
    pub status: VendorStatus,
    pub allowed_categories: Vec<Symbol>,
    pub metadata_uri: String,
    pub total_redeemed: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VoucherIssueRequest {
    pub beneficiary: Address,
    pub amount: i128,
    pub category: Symbol,
    pub expires_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DonorContribution {
    pub donor: Address,
    pub program_id: u64,
    pub amount: i128,
    pub refunded: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ContractStats {
    pub total_programs: u64,
    pub total_vouchers: u64,
    pub total_vendors: u32,
    pub total_funded_volume: i128,
    pub total_released_volume: i128,
    pub total_redeemed_volume: i128,
}
