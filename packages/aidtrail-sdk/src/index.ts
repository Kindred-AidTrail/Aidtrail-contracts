import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
  }
} as const


export interface Vendor {
  address: string;
  allowed_categories: Array<string>;
  metadata_uri: string;
  status: VendorStatus;
  total_redeemed: i128;
}


export interface Program {
  created_at: u64;
  id: u64;
  metadata_uri: string;
  ngo: string;
  refundable_pool: i128;
  status: ProgramStatus;
  token: string;
  total_allocated: i128;
  total_funded: i128;
  total_released: i128;
}


export interface Voucher {
  amount: i128;
  beneficiary: string;
  category: string;
  created_at: u64;
  expires_at: u64;
  id: u64;
  program_id: u64;
  status: VoucherStatus;
}


export interface Milestone {
  amount: i128;
  approvals: Array<string>;
  description_uri: string;
  id: u32;
  program_id: u64;
  required_approvals: u32;
  status: MilestoneStatus;
  verifiers: Array<string>;
}

export enum VendorStatus {
  Active = 0,
  Suspended = 1,
}


export interface ContractStats {
  total_funded_volume: i128;
  total_programs: u64;
  total_redeemed_volume: i128;
  total_released_volume: i128;
  total_vendors: u32;
  total_vouchers: u64;
}

export enum ProgramStatus {
  Active = 0,
  Completed = 1,
  Cancelled = 2,
}

export enum VoucherStatus {
  Active = 0,
  Redeemed = 1,
  Reclaimed = 2,
  Cancelled = 3,
}

export enum MilestoneStatus {
  Pending = 0,
  Approved = 1,
  Released = 2,
}


export interface DonorContribution {
  amount: i128;
  donor: string;
  program_id: u64;
  refunded: boolean;
}


export interface VoucherIssueRequest {
  amount: i128;
  beneficiary: string;
  category: string;
  expires_at: u64;
}

export const Errors = {
  1: {message:"NotInitialized"},

  2: {message:"AlreadyInitialized"},

  3: {message:"Unauthorized"},

  4: {message:"ContractPaused"},

  5: {message:"InvalidAmount"},

  6: {message:"ProgramNotFound"},

  7: {message:"ProgramNotActive"},

  8: {message:"ProgramCancelled"},

  9: {message:"MilestoneNotFound"},

  10: {message:"MilestoneAlreadyApproved"},

  11: {message:"MilestoneAlreadyReleased"},

  12: {message:"MilestoneNotApproved"},

  13: {message:"InsufficientProgramFunds"},

  14: {message:"VerifierNotAuthorized"},

  15: {message:"DuplicateVerifierApproval"},

  16: {message:"InsufficientApprovals"},

  17: {message:"VendorNotFound"},

  18: {message:"VendorNotActive"},

  19: {message:"VendorCategoryNotAllowed"},

  20: {message:"VoucherNotFound"},

  21: {message:"VoucherNotActive"},

  22: {message:"VoucherExpired"},

  23: {message:"VoucherNotExpired"},

  24: {message:"ArithmeticOverflow"},

  25: {message:"EmptyBatch"},

  26: {message:"BatchSizeExceeded"},

  27: {message:"NothingToRefund"},

  28: {message:"AlreadyRefunded"},

  29: {message:"DonorContributionNotFound"},

  30: {message:"InvalidMilestoneConfig"},

  31: {message:"InvalidExpiration"}
}
export type DataKey = {tag: "Admin", values: void} | {tag: "EmergencyAdmin", values: void} | {tag: "Paused", values: void} | {tag: "ProgramCount", values: void} | {tag: "VoucherCount", values: void} | {tag: "VendorCount", values: void} | {tag: "Program", values: readonly [u64]} | {tag: "Milestone", values: readonly [u64, u32]} | {tag: "MilestoneCount", values: readonly [u64]} | {tag: "DonorContribution", values: readonly [u64, string]} | {tag: "Vendor", values: readonly [string]} | {tag: "Voucher", values: readonly [u64]};


export interface Client {
  /**
   * Construct and simulate a redeem transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Redeem an active voucher at an approved vendor matching the voucher's category.
   */
  redeem: ({beneficiary, voucher_id, vendor_address}: {beneficiary: string, voucher_id: u64, vendor_address: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query current contract admin address.
   */
  get_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

  /**
   * Construct and simulate a is_paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query whether contract operations are currently paused.
   */
  is_paused: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a set_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Transfer contract administrative ownership.
   */
  set_admin: ({current_admin, new_admin}: {current_admin: string, new_admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_vendor transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query vendor profile by vendor address.
   */
  get_vendor: ({vendor}: {vendor: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Vendor>>>

  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the AidTrail contract with an admin and optional emergency admin.
   */
  initialize: ({admin, emergency_admin}: {admin: string, emergency_admin: Option<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a set_paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Toggle emergency circuit breaker (pause/unpause).
   */
  set_paused: ({caller, paused}: {caller: string, paused: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_program transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query a program by its ID.
   */
  get_program: ({program_id}: {program_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Program>>>

  /**
   * Construct and simulate a get_voucher transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query a voucher by its unique ID.
   */
  get_voucher: ({voucher_id}: {voucher_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Voucher>>>

  /**
   * Construct and simulate a fund_program transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Donors fund an aid program by transferring tokens into contract custody.
   */
  fund_program: ({donor, program_id, amount}: {donor: string, program_id: u64, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a add_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add a milestone to an aid program with M-of-N verifier approval requirements.
   */
  add_milestone: ({caller, program_id, amount, description_uri, required_approvals, verifiers}: {caller: string, program_id: u64, amount: i128, description_uri: string, required_approvals: u32, verifiers: Array<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u32>>>

  /**
   * Construct and simulate a get_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query a milestone by program ID and milestone ID.
   */
  get_milestone: ({program_id, milestone_id}: {program_id: u64, milestone_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Milestone>>>

  /**
   * Construct and simulate a issue_voucher transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Issue a voucher to a beneficiary against unlocked milestone funds.
   */
  issue_voucher: ({caller, program_id, beneficiary, amount, category, expires_at}: {caller: string, program_id: u64, beneficiary: string, amount: i128, category: string, expires_at: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a remove_vendor transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Suspend or remove a vendor from participating in voucher redemptions.
   */
  remove_vendor: ({caller, vendor_address}: {caller: string, vendor_address: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a cancel_program transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Cancel an active aid program, freezing unreleased funds into a refundable donor pool.
   */
  cancel_program: ({caller, program_id}: {caller: string, program_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a create_program transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new aid program with specified payment token and metadata.
   */
  create_program: ({ngo, token, metadata_uri}: {ngo: string, token: string, metadata_uri: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

  /**
   * Construct and simulate a reclaim_expired transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Reclaim funds from an expired, unredeemed voucher back to the program allocation pool.
   */
  reclaim_expired: ({caller, voucher_id}: {caller: string, voucher_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a register_vendor transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Register a vendor with allowed redemption categories and metadata.
   */
  register_vendor: ({caller, vendor_address, allowed_categories, metadata_uri}: {caller: string, vendor_address: string, allowed_categories: Array<string>, metadata_uri: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_all_programs transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve paginated list of programs.
   */
  get_all_programs: ({start_id, limit}: {start_id: u64, limit: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<Program>>>

  /**
   * Construct and simulate a get_vendor_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query total number of registered vendors.
   */
  get_vendor_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a approve_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Submit independent verifier approval for a milestone with evidence.
   */
  approve_milestone: ({verifier, program_id, milestone_id, _evidence_uri}: {verifier: string, program_id: u64, milestone_id: u32, _evidence_uri: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_program_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query total number of created programs.
   */
  get_program_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_voucher_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query total number of vouchers created.
   */
  get_voucher_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a is_vendor_allowed transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if a vendor is active and permitted to redeem vouchers of a given category.
   */
  is_vendor_allowed: ({vendor, category}: {vendor: string, category: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a release_milestone transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Release an approved milestone, validating funded >= released accounting invariant.
   */
  release_milestone: ({caller, program_id, milestone_id}: {caller: string, program_id: u64, milestone_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a set_vendor_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Update status of a vendor (Active or Suspended).
   */
  set_vendor_status: ({caller, vendor_address, status}: {caller: string, vendor_address: string, status: VendorStatus}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a claim_donor_refund transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Claim proportional refund of unreleased program capital as a donor.
   */
  claim_donor_refund: ({donor, program_id}: {donor: string, program_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<i128>>>

  /**
   * Construct and simulate a get_contract_stats transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query global platform statistics for dashboard and audit explorer.
   */
  get_contract_stats: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<ContractStats>>

  /**
   * Construct and simulate a get_emergency_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query current contract emergency admin address.
   */
  get_emergency_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

  /**
   * Construct and simulate a get_milestone_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query the total number of milestones for a given program.
   */
  get_milestone_count: ({program_id}: {program_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a set_emergency_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Set or update the emergency admin address.
   */
  set_emergency_admin: ({admin, emergency_admin}: {admin: string, emergency_admin: Option<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a batch_issue_vouchers transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Batch issue vouchers to beneficiaries in a single transaction.
   */
  batch_issue_vouchers: ({caller, program_id, vouchers}: {caller: string, program_id: u64, vouchers: Array<VoucherIssueRequest>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<Array<u64>>>>

  /**
   * Construct and simulate a is_milestone_approved transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if a milestone has reached its required approval threshold.
   */
  is_milestone_approved: ({program_id, milestone_id}: {program_id: u64, milestone_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_donor_contribution transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Query contribution record for a specific donor and program.
   */
  get_donor_contribution: ({program_id, donor}: {program_id: u64, donor: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<DonorContribution>>>

  /**
   * Construct and simulate a get_program_milestones transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve all milestones belonging to a program.
   */
  get_program_milestones: ({program_id}: {program_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<Milestone>>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAE9SZWRlZW0gYW4gYWN0aXZlIHZvdWNoZXIgYXQgYW4gYXBwcm92ZWQgdmVuZG9yIG1hdGNoaW5nIHRoZSB2b3VjaGVyJ3MgY2F0ZWdvcnkuAAAAAAZyZWRlZW0AAAAAAAMAAAAAAAAAC2JlbmVmaWNpYXJ5AAAAABMAAAAAAAAACnZvdWNoZXJfaWQAAAAAAAYAAAAAAAAADnZlbmRvcl9hZGRyZXNzAAAAAAATAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAACVRdWVyeSBjdXJyZW50IGNvbnRyYWN0IGFkbWluIGFkZHJlc3MuAAAAAAAACWdldF9hZG1pbgAAAAAAAAAAAAABAAAD6AAAABM=",
        "AAAAAAAAADdRdWVyeSB3aGV0aGVyIGNvbnRyYWN0IG9wZXJhdGlvbnMgYXJlIGN1cnJlbnRseSBwYXVzZWQuAAAAAAlpc19wYXVzZWQAAAAAAAAAAAAAAQAAAAE=",
        "AAAAAAAAACtUcmFuc2ZlciBjb250cmFjdCBhZG1pbmlzdHJhdGl2ZSBvd25lcnNoaXAuAAAAAAlzZXRfYWRtaW4AAAAAAAACAAAAAAAAAA1jdXJyZW50X2FkbWluAAAAAAAAEwAAAAAAAAAJbmV3X2FkbWluAAAAAAAAEwAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAACdRdWVyeSB2ZW5kb3IgcHJvZmlsZSBieSB2ZW5kb3IgYWRkcmVzcy4AAAAACmdldF92ZW5kb3IAAAAAAAEAAAAAAAAABnZlbmRvcgAAAAAAEwAAAAEAAAPpAAAH0AAAAAZWZW5kb3IAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAExJbml0aWFsaXplIHRoZSBBaWRUcmFpbCBjb250cmFjdCB3aXRoIGFuIGFkbWluIGFuZCBvcHRpb25hbCBlbWVyZ2VuY3kgYWRtaW4uAAAACmluaXRpYWxpemUAAAAAAAIAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAPZW1lcmdlbmN5X2FkbWluAAAAA+gAAAATAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAADFUb2dnbGUgZW1lcmdlbmN5IGNpcmN1aXQgYnJlYWtlciAocGF1c2UvdW5wYXVzZSkuAAAAAAAACnNldF9wYXVzZWQAAAAAAAIAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAGcGF1c2VkAAAAAAABAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAABpRdWVyeSBhIHByb2dyYW0gYnkgaXRzIElELgAAAAAAC2dldF9wcm9ncmFtAAAAAAEAAAAAAAAACnByb2dyYW1faWQAAAAAAAYAAAABAAAD6QAAB9AAAAAHUHJvZ3JhbQAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAACFRdWVyeSBhIHZvdWNoZXIgYnkgaXRzIHVuaXF1ZSBJRC4AAAAAAAALZ2V0X3ZvdWNoZXIAAAAAAQAAAAAAAAAKdm91Y2hlcl9pZAAAAAAABgAAAAEAAAPpAAAH0AAAAAdWb3VjaGVyAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEhEb25vcnMgZnVuZCBhbiBhaWQgcHJvZ3JhbSBieSB0cmFuc2ZlcnJpbmcgdG9rZW5zIGludG8gY29udHJhY3QgY3VzdG9keS4AAAAMZnVuZF9wcm9ncmFtAAAAAwAAAAAAAAAFZG9ub3IAAAAAAAATAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAE1BZGQgYSBtaWxlc3RvbmUgdG8gYW4gYWlkIHByb2dyYW0gd2l0aCBNLW9mLU4gdmVyaWZpZXIgYXBwcm92YWwgcmVxdWlyZW1lbnRzLgAAAAAAAA1hZGRfbWlsZXN0b25lAAAAAAAABgAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAAD2Rlc2NyaXB0aW9uX3VyaQAAAAAQAAAAAAAAABJyZXF1aXJlZF9hcHByb3ZhbHMAAAAAAAQAAAAAAAAACXZlcmlmaWVycwAAAAAAA+oAAAATAAAAAQAAA+kAAAAEAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAADFRdWVyeSBhIG1pbGVzdG9uZSBieSBwcm9ncmFtIElEIGFuZCBtaWxlc3RvbmUgSUQuAAAAAAAADWdldF9taWxlc3RvbmUAAAAAAAACAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAxtaWxlc3RvbmVfaWQAAAAEAAAAAQAAA+kAAAfQAAAACU1pbGVzdG9uZQAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEJJc3N1ZSBhIHZvdWNoZXIgdG8gYSBiZW5lZmljaWFyeSBhZ2FpbnN0IHVubG9ja2VkIG1pbGVzdG9uZSBmdW5kcy4AAAAAAA1pc3N1ZV92b3VjaGVyAAAAAAAABgAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAtiZW5lZmljaWFyeQAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAACGNhdGVnb3J5AAAAEQAAAAAAAAAKZXhwaXJlc19hdAAAAAAABgAAAAEAAAPpAAAABgAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEVTdXNwZW5kIG9yIHJlbW92ZSBhIHZlbmRvciBmcm9tIHBhcnRpY2lwYXRpbmcgaW4gdm91Y2hlciByZWRlbXB0aW9ucy4AAAAAAAANcmVtb3ZlX3ZlbmRvcgAAAAAAAAIAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAOdmVuZG9yX2FkZHJlc3MAAAAAABMAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAFVDYW5jZWwgYW4gYWN0aXZlIGFpZCBwcm9ncmFtLCBmcmVlemluZyB1bnJlbGVhc2VkIGZ1bmRzIGludG8gYSByZWZ1bmRhYmxlIGRvbm9yIHBvb2wuAAAAAAAADmNhbmNlbF9wcm9ncmFtAAAAAAACAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACnByb2dyYW1faWQAAAAAAAYAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAENDcmVhdGUgYSBuZXcgYWlkIHByb2dyYW0gd2l0aCBzcGVjaWZpZWQgcGF5bWVudCB0b2tlbiBhbmQgbWV0YWRhdGEuAAAAAA5jcmVhdGVfcHJvZ3JhbQAAAAAAAwAAAAAAAAADbmdvAAAAABMAAAAAAAAABXRva2VuAAAAAAAAEwAAAAAAAAAMbWV0YWRhdGFfdXJpAAAAEAAAAAEAAAPpAAAABgAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAFZSZWNsYWltIGZ1bmRzIGZyb20gYW4gZXhwaXJlZCwgdW5yZWRlZW1lZCB2b3VjaGVyIGJhY2sgdG8gdGhlIHByb2dyYW0gYWxsb2NhdGlvbiBwb29sLgAAAAAAD3JlY2xhaW1fZXhwaXJlZAAAAAACAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACnZvdWNoZXJfaWQAAAAAAAYAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAAEJSZWdpc3RlciBhIHZlbmRvciB3aXRoIGFsbG93ZWQgcmVkZW1wdGlvbiBjYXRlZ29yaWVzIGFuZCBtZXRhZGF0YS4AAAAAAA9yZWdpc3Rlcl92ZW5kb3IAAAAABAAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAA52ZW5kb3JfYWRkcmVzcwAAAAAAEwAAAAAAAAASYWxsb3dlZF9jYXRlZ29yaWVzAAAAAAPqAAAAEQAAAAAAAAAMbWV0YWRhdGFfdXJpAAAAEAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAADUNvbnRyYWN0RXJyb3IAAAA=",
        "AAAAAAAAACRSZXRyaWV2ZSBwYWdpbmF0ZWQgbGlzdCBvZiBwcm9ncmFtcy4AAAAQZ2V0X2FsbF9wcm9ncmFtcwAAAAIAAAAAAAAACHN0YXJ0X2lkAAAABgAAAAAAAAAFbGltaXQAAAAAAAAEAAAAAQAAA+oAAAfQAAAAB1Byb2dyYW0A",
        "AAAAAAAAAClRdWVyeSB0b3RhbCBudW1iZXIgb2YgcmVnaXN0ZXJlZCB2ZW5kb3JzLgAAAAAAABBnZXRfdmVuZG9yX2NvdW50AAAAAAAAAAEAAAAE",
        "AAAAAAAAAENTdWJtaXQgaW5kZXBlbmRlbnQgdmVyaWZpZXIgYXBwcm92YWwgZm9yIGEgbWlsZXN0b25lIHdpdGggZXZpZGVuY2UuAAAAABFhcHByb3ZlX21pbGVzdG9uZQAAAAAAAAQAAAAAAAAACHZlcmlmaWVyAAAAEwAAAAAAAAAKcHJvZ3JhbV9pZAAAAAAABgAAAAAAAAAMbWlsZXN0b25lX2lkAAAABAAAAAAAAAANX2V2aWRlbmNlX3VyaQAAAAAAABAAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAACdRdWVyeSB0b3RhbCBudW1iZXIgb2YgY3JlYXRlZCBwcm9ncmFtcy4AAAAAEWdldF9wcm9ncmFtX2NvdW50AAAAAAAAAAAAAAEAAAAG",
        "AAAAAAAAACdRdWVyeSB0b3RhbCBudW1iZXIgb2Ygdm91Y2hlcnMgY3JlYXRlZC4AAAAAEWdldF92b3VjaGVyX2NvdW50AAAAAAAAAAAAAAEAAAAG",
        "AAAAAAAAAFFDaGVjayBpZiBhIHZlbmRvciBpcyBhY3RpdmUgYW5kIHBlcm1pdHRlZCB0byByZWRlZW0gdm91Y2hlcnMgb2YgYSBnaXZlbiBjYXRlZ29yeS4AAAAAAAARaXNfdmVuZG9yX2FsbG93ZWQAAAAAAAACAAAAAAAAAAZ2ZW5kb3IAAAAAABMAAAAAAAAACGNhdGVnb3J5AAAAEQAAAAEAAAAB",
        "AAAAAAAAAFJSZWxlYXNlIGFuIGFwcHJvdmVkIG1pbGVzdG9uZSwgdmFsaWRhdGluZyBmdW5kZWQgPj0gcmVsZWFzZWQgYWNjb3VudGluZyBpbnZhcmlhbnQuAAAAAAARcmVsZWFzZV9taWxlc3RvbmUAAAAAAAADAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACnByb2dyYW1faWQAAAAAAAYAAAAAAAAADG1pbGVzdG9uZV9pZAAAAAQAAAABAAAD6QAAA+0AAAAAAAAH0AAAAA1Db250cmFjdEVycm9yAAAA",
        "AAAAAAAAADBVcGRhdGUgc3RhdHVzIG9mIGEgdmVuZG9yIChBY3RpdmUgb3IgU3VzcGVuZGVkKS4AAAARc2V0X3ZlbmRvcl9zdGF0dXMAAAAAAAADAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAADnZlbmRvcl9hZGRyZXNzAAAAAAATAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAAMVmVuZG9yU3RhdHVzAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAENDbGFpbSBwcm9wb3J0aW9uYWwgcmVmdW5kIG9mIHVucmVsZWFzZWQgcHJvZ3JhbSBjYXBpdGFsIGFzIGEgZG9ub3IuAAAAABJjbGFpbV9kb25vcl9yZWZ1bmQAAAAAAAIAAAAAAAAABWRvbm9yAAAAAAAAEwAAAAAAAAAKcHJvZ3JhbV9pZAAAAAAABgAAAAEAAAPpAAAACwAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEJRdWVyeSBnbG9iYWwgcGxhdGZvcm0gc3RhdGlzdGljcyBmb3IgZGFzaGJvYXJkIGFuZCBhdWRpdCBleHBsb3Jlci4AAAAAABJnZXRfY29udHJhY3Rfc3RhdHMAAAAAAAAAAAABAAAH0AAAAA1Db250cmFjdFN0YXRzAAAA",
        "AAAAAAAAAC9RdWVyeSBjdXJyZW50IGNvbnRyYWN0IGVtZXJnZW5jeSBhZG1pbiBhZGRyZXNzLgAAAAATZ2V0X2VtZXJnZW5jeV9hZG1pbgAAAAAAAAAAAQAAA+gAAAAT",
        "AAAAAAAAADlRdWVyeSB0aGUgdG90YWwgbnVtYmVyIG9mIG1pbGVzdG9uZXMgZm9yIGEgZ2l2ZW4gcHJvZ3JhbS4AAAAAAAATZ2V0X21pbGVzdG9uZV9jb3VudAAAAAABAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAQAAAAQ=",
        "AAAAAAAAACpTZXQgb3IgdXBkYXRlIHRoZSBlbWVyZ2VuY3kgYWRtaW4gYWRkcmVzcy4AAAAAABNzZXRfZW1lcmdlbmN5X2FkbWluAAAAAAIAAAAAAAAABWFkbWluAAAAAAAAEwAAAAAAAAAPZW1lcmdlbmN5X2FkbWluAAAAA+gAAAATAAAAAQAAA+kAAAPtAAAAAAAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAD5CYXRjaCBpc3N1ZSB2b3VjaGVycyB0byBiZW5lZmljaWFyaWVzIGluIGEgc2luZ2xlIHRyYW5zYWN0aW9uLgAAAAAAFGJhdGNoX2lzc3VlX3ZvdWNoZXJzAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAh2b3VjaGVycwAAA+oAAAfQAAAAE1ZvdWNoZXJJc3N1ZVJlcXVlc3QAAAAAAQAAA+kAAAPqAAAABgAAB9AAAAANQ29udHJhY3RFcnJvcgAAAA==",
        "AAAAAAAAAEFDaGVjayBpZiBhIG1pbGVzdG9uZSBoYXMgcmVhY2hlZCBpdHMgcmVxdWlyZWQgYXBwcm92YWwgdGhyZXNob2xkLgAAAAAAABVpc19taWxlc3RvbmVfYXBwcm92ZWQAAAAAAAACAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAxtaWxlc3RvbmVfaWQAAAAEAAAAAQAAAAE=",
        "AAAAAAAAADtRdWVyeSBjb250cmlidXRpb24gcmVjb3JkIGZvciBhIHNwZWNpZmljIGRvbm9yIGFuZCBwcm9ncmFtLgAAAAAWZ2V0X2Rvbm9yX2NvbnRyaWJ1dGlvbgAAAAAAAgAAAAAAAAAKcHJvZ3JhbV9pZAAAAAAABgAAAAAAAAAFZG9ub3IAAAAAAAATAAAAAQAAA+gAAAfQAAAAEURvbm9yQ29udHJpYnV0aW9uAAAA",
        "AAAAAAAAAC9SZXRyaWV2ZSBhbGwgbWlsZXN0b25lcyBiZWxvbmdpbmcgdG8gYSBwcm9ncmFtLgAAAAAWZ2V0X3Byb2dyYW1fbWlsZXN0b25lcwAAAAAAAQAAAAAAAAAKcHJvZ3JhbV9pZAAAAAAABgAAAAEAAAPqAAAH0AAAAAlNaWxlc3RvbmUAAAA=",
        "AAAAAQAAAAAAAAAAAAAABlZlbmRvcgAAAAAABQAAAAAAAAAHYWRkcmVzcwAAAAATAAAAAAAAABJhbGxvd2VkX2NhdGVnb3JpZXMAAAAAA+oAAAARAAAAAAAAAAxtZXRhZGF0YV91cmkAAAAQAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAAMVmVuZG9yU3RhdHVzAAAAAAAAAA50b3RhbF9yZWRlZW1lZAAAAAAACw==",
        "AAAAAQAAAAAAAAAAAAAAB1Byb2dyYW0AAAAACgAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAACaWQAAAAAAAYAAAAAAAAADG1ldGFkYXRhX3VyaQAAABAAAAAAAAAAA25nbwAAAAATAAAAAAAAAA9yZWZ1bmRhYmxlX3Bvb2wAAAAACwAAAAAAAAAGc3RhdHVzAAAAAAfQAAAADVByb2dyYW1TdGF0dXMAAAAAAAAAAAAABXRva2VuAAAAAAAAEwAAAAAAAAAPdG90YWxfYWxsb2NhdGVkAAAAAAsAAAAAAAAADHRvdGFsX2Z1bmRlZAAAAAsAAAAAAAAADnRvdGFsX3JlbGVhc2VkAAAAAAAL",
        "AAAAAQAAAAAAAAAAAAAAB1ZvdWNoZXIAAAAACAAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAtiZW5lZmljaWFyeQAAAAATAAAAAAAAAAhjYXRlZ29yeQAAABEAAAAAAAAACmNyZWF0ZWRfYXQAAAAAAAYAAAAAAAAACmV4cGlyZXNfYXQAAAAAAAYAAAAAAAAAAmlkAAAAAAAGAAAAAAAAAApwcm9ncmFtX2lkAAAAAAAGAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAANVm91Y2hlclN0YXR1cwAAAA==",
        "AAAAAQAAAAAAAAAAAAAACU1pbGVzdG9uZQAAAAAAAAgAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAJYXBwcm92YWxzAAAAAAAD6gAAABMAAAAAAAAAD2Rlc2NyaXB0aW9uX3VyaQAAAAAQAAAAAAAAAAJpZAAAAAAABAAAAAAAAAAKcHJvZ3JhbV9pZAAAAAAABgAAAAAAAAAScmVxdWlyZWRfYXBwcm92YWxzAAAAAAAEAAAAAAAAAAZzdGF0dXMAAAAAB9AAAAAPTWlsZXN0b25lU3RhdHVzAAAAAAAAAAAJdmVyaWZpZXJzAAAAAAAD6gAAABM=",
        "AAAAAwAAAAAAAAAAAAAADFZlbmRvclN0YXR1cwAAAAIAAAAAAAAABkFjdGl2ZQAAAAAAAAAAAAAAAAAJU3VzcGVuZGVkAAAAAAAAAQ==",
        "AAAAAQAAAAAAAAAAAAAADUNvbnRyYWN0U3RhdHMAAAAAAAAGAAAAAAAAABN0b3RhbF9mdW5kZWRfdm9sdW1lAAAAAAsAAAAAAAAADnRvdGFsX3Byb2dyYW1zAAAAAAAGAAAAAAAAABV0b3RhbF9yZWRlZW1lZF92b2x1bWUAAAAAAAALAAAAAAAAABV0b3RhbF9yZWxlYXNlZF92b2x1bWUAAAAAAAALAAAAAAAAAA10b3RhbF92ZW5kb3JzAAAAAAAABAAAAAAAAAAOdG90YWxfdm91Y2hlcnMAAAAAAAY=",
        "AAAAAwAAAAAAAAAAAAAADVByb2dyYW1TdGF0dXMAAAAAAAADAAAAAAAAAAZBY3RpdmUAAAAAAAAAAAAAAAAACUNvbXBsZXRlZAAAAAAAAAEAAAAAAAAACUNhbmNlbGxlZAAAAAAAAAI=",
        "AAAAAwAAAAAAAAAAAAAADVZvdWNoZXJTdGF0dXMAAAAAAAAEAAAAAAAAAAZBY3RpdmUAAAAAAAAAAAAAAAAACFJlZGVlbWVkAAAAAQAAAAAAAAAJUmVjbGFpbWVkAAAAAAAAAgAAAAAAAAAJQ2FuY2VsbGVkAAAAAAAAAw==",
        "AAAAAwAAAAAAAAAAAAAAD01pbGVzdG9uZVN0YXR1cwAAAAADAAAAAAAAAAdQZW5kaW5nAAAAAAAAAAAAAAAACEFwcHJvdmVkAAAAAQAAAAAAAAAIUmVsZWFzZWQAAAAC",
        "AAAAAQAAAAAAAAAAAAAAEURvbm9yQ29udHJpYnV0aW9uAAAAAAAABAAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAVkb25vcgAAAAAAABMAAAAAAAAACnByb2dyYW1faWQAAAAAAAYAAAAAAAAACHJlZnVuZGVkAAAAAQ==",
        "AAAAAQAAAAAAAAAAAAAAE1ZvdWNoZXJJc3N1ZVJlcXVlc3QAAAAABAAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAtiZW5lZmljaWFyeQAAAAATAAAAAAAAAAhjYXRlZ29yeQAAABEAAAAAAAAACmV4cGlyZXNfYXQAAAAAAAY=",
        "AAAABAAAAAAAAAAAAAAADUNvbnRyYWN0RXJyb3IAAAAAAAAfAAAAAAAAAA5Ob3RJbml0aWFsaXplZAAAAAAAAQAAAAAAAAASQWxyZWFkeUluaXRpYWxpemVkAAAAAAACAAAAAAAAAAxVbmF1dGhvcml6ZWQAAAADAAAAAAAAAA5Db250cmFjdFBhdXNlZAAAAAAABAAAAAAAAAANSW52YWxpZEFtb3VudAAAAAAAAAUAAAAAAAAAD1Byb2dyYW1Ob3RGb3VuZAAAAAAGAAAAAAAAABBQcm9ncmFtTm90QWN0aXZlAAAABwAAAAAAAAAQUHJvZ3JhbUNhbmNlbGxlZAAAAAgAAAAAAAAAEU1pbGVzdG9uZU5vdEZvdW5kAAAAAAAACQAAAAAAAAAYTWlsZXN0b25lQWxyZWFkeUFwcHJvdmVkAAAACgAAAAAAAAAYTWlsZXN0b25lQWxyZWFkeVJlbGVhc2VkAAAACwAAAAAAAAAUTWlsZXN0b25lTm90QXBwcm92ZWQAAAAMAAAAAAAAABhJbnN1ZmZpY2llbnRQcm9ncmFtRnVuZHMAAAANAAAAAAAAABVWZXJpZmllck5vdEF1dGhvcml6ZWQAAAAAAAAOAAAAAAAAABlEdXBsaWNhdGVWZXJpZmllckFwcHJvdmFsAAAAAAAADwAAAAAAAAAVSW5zdWZmaWNpZW50QXBwcm92YWxzAAAAAAAAEAAAAAAAAAAOVmVuZG9yTm90Rm91bmQAAAAAABEAAAAAAAAAD1ZlbmRvck5vdEFjdGl2ZQAAAAASAAAAAAAAABhWZW5kb3JDYXRlZ29yeU5vdEFsbG93ZWQAAAATAAAAAAAAAA9Wb3VjaGVyTm90Rm91bmQAAAAAFAAAAAAAAAAQVm91Y2hlck5vdEFjdGl2ZQAAABUAAAAAAAAADlZvdWNoZXJFeHBpcmVkAAAAAAAWAAAAAAAAABFWb3VjaGVyTm90RXhwaXJlZAAAAAAAABcAAAAAAAAAEkFyaXRobWV0aWNPdmVyZmxvdwAAAAAAGAAAAAAAAAAKRW1wdHlCYXRjaAAAAAAAGQAAAAAAAAARQmF0Y2hTaXplRXhjZWVkZWQAAAAAAAAaAAAAAAAAAA9Ob3RoaW5nVG9SZWZ1bmQAAAAAGwAAAAAAAAAPQWxyZWFkeVJlZnVuZGVkAAAAABwAAAAAAAAAGURvbm9yQ29udHJpYnV0aW9uTm90Rm91bmQAAAAAAAAdAAAAAAAAABZJbnZhbGlkTWlsZXN0b25lQ29uZmlnAAAAAAAeAAAAAAAAABFJbnZhbGlkRXhwaXJhdGlvbgAAAAAAAB8=",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAADAAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAORW1lcmdlbmN5QWRtaW4AAAAAAAAAAAAAAAAABlBhdXNlZAAAAAAAAAAAAAAAAAAMUHJvZ3JhbUNvdW50AAAAAAAAAAAAAAAMVm91Y2hlckNvdW50AAAAAAAAAAAAAAALVmVuZG9yQ291bnQAAAAAAQAAAAAAAAAHUHJvZ3JhbQAAAAABAAAABgAAAAEAAAAAAAAACU1pbGVzdG9uZQAAAAAAAAIAAAAGAAAABAAAAAEAAAAAAAAADk1pbGVzdG9uZUNvdW50AAAAAAABAAAABgAAAAEAAAAAAAAAEURvbm9yQ29udHJpYnV0aW9uAAAAAAAAAgAAAAYAAAATAAAAAQAAAAAAAAAGVmVuZG9yAAAAAAABAAAAEwAAAAEAAAAAAAAAB1ZvdWNoZXIAAAAAAQAAAAY=" ]),
      options
    )
  }
  public readonly fromJSON = {
    redeem: this.txFromJSON<Result<void>>,
        get_admin: this.txFromJSON<Option<string>>,
        is_paused: this.txFromJSON<boolean>,
        set_admin: this.txFromJSON<Result<void>>,
        get_vendor: this.txFromJSON<Result<Vendor>>,
        initialize: this.txFromJSON<Result<void>>,
        set_paused: this.txFromJSON<Result<void>>,
        get_program: this.txFromJSON<Result<Program>>,
        get_voucher: this.txFromJSON<Result<Voucher>>,
        fund_program: this.txFromJSON<Result<void>>,
        add_milestone: this.txFromJSON<Result<u32>>,
        get_milestone: this.txFromJSON<Result<Milestone>>,
        issue_voucher: this.txFromJSON<Result<u64>>,
        remove_vendor: this.txFromJSON<Result<void>>,
        cancel_program: this.txFromJSON<Result<void>>,
        create_program: this.txFromJSON<Result<u64>>,
        reclaim_expired: this.txFromJSON<Result<void>>,
        register_vendor: this.txFromJSON<Result<void>>,
        get_all_programs: this.txFromJSON<Array<Program>>,
        get_vendor_count: this.txFromJSON<u32>,
        approve_milestone: this.txFromJSON<Result<void>>,
        get_program_count: this.txFromJSON<u64>,
        get_voucher_count: this.txFromJSON<u64>,
        is_vendor_allowed: this.txFromJSON<boolean>,
        release_milestone: this.txFromJSON<Result<void>>,
        set_vendor_status: this.txFromJSON<Result<void>>,
        claim_donor_refund: this.txFromJSON<Result<i128>>,
        get_contract_stats: this.txFromJSON<ContractStats>,
        get_emergency_admin: this.txFromJSON<Option<string>>,
        get_milestone_count: this.txFromJSON<u32>,
        set_emergency_admin: this.txFromJSON<Result<void>>,
        batch_issue_vouchers: this.txFromJSON<Result<Array<u64>>>,
        is_milestone_approved: this.txFromJSON<boolean>,
        get_donor_contribution: this.txFromJSON<Option<DonorContribution>>,
        get_program_milestones: this.txFromJSON<Array<Milestone>>
  }
}