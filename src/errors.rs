use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    ContractPaused = 4,
    InvalidAmount = 5,
    ProgramNotFound = 6,
    ProgramNotActive = 7,
    ProgramCancelled = 8,
    MilestoneNotFound = 9,
    MilestoneAlreadyApproved = 10,
    MilestoneAlreadyReleased = 11,
    MilestoneNotApproved = 12,
    InsufficientProgramFunds = 13,
    VerifierNotAuthorized = 14,
    DuplicateVerifierApproval = 15,
    InsufficientApprovals = 16,
    VendorNotFound = 17,
    VendorNotActive = 18,
    VendorCategoryNotAllowed = 19,
    VoucherNotFound = 20,
    VoucherNotActive = 21,
    VoucherExpired = 22,
    VoucherNotExpired = 23,
    ArithmeticOverflow = 24,
    EmptyBatch = 25,
    BatchSizeExceeded = 26,
    NothingToRefund = 27,
    AlreadyRefunded = 28,
    DonorContributionNotFound = 29,
    InvalidMilestoneConfig = 30,
    InvalidExpiration = 31,
}
