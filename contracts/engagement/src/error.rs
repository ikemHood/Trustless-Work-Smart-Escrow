use soroban_sdk::contracterror;
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
#[contracterror]
pub enum ContractError {
    EscrowNotFunded = 1,
    AmountCannotBeZero = 2,
    EscrowAlreadyInitialized = 3,
    OnlySignerCanFundEscrow = 4,
    EscrowAlreadyFunded = 5,
    EscrowFullyFunded = 6,
    SignerInsufficientFunds = 7,
    NotEnoughAllowance = 8,
    OnlySignerCanCompleteEscrow = 9,
    EscrowAlreadyCompleted = 10,
    SignerInsufficientFundsToComplete = 11,
    OnlyServiceProviderCanCancelEscrow = 12,
    EscrowAlreadyCancelled = 13,
    OnlySignerCanRequestRefund = 14,
    EscrowNotCancelled = 15,
    NoFundsToRefund = 16,
    ContractHasInsufficientBalance = 17,
    EscrowNotFound = 18,
    OnlyServiceProviderCanClaimEarnings = 19,
    EscrowNotCompleted = 20,
    EscrowBalanceNotSufficienteToSendEarnings = 21,
    ContractInsufficientFunds = 22,
    OnlyPlatformAddressExecuteThisFunction = 23,
    EscrowNotInitialized = 24,
    OnlyServiceProviderChangeMilstoneStatus = 25,
    NoMileStoneDefined = 26,
    InvalidMileStoneIndex = 27,
    OnlyClientChangeMilstoneFlag = 28,
    OnlyDisputeResolverCanExecuteThisFunction = 29,
    EscrowAlreadyInDispute = 30,
    EscrowNotInDispute = 31,
    InsufficientFundsForResolution = 32,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::EscrowNotFunded => write!(f, "Escrow not funded"),
            ContractError::AmountCannotBeZero => write!(f, "Amount cannot be zero"),
            ContractError::EscrowAlreadyInitialized => write!(f, "Escrow already initialized"),
            ContractError::OnlySignerCanFundEscrow => write!(f, "Only the signer can fund the escrow"),
            ContractError::EscrowAlreadyFunded => write!(f, "Escrow already funded"),
            ContractError::EscrowFullyFunded => write!(f, "This escrow is already fully funded"),
            ContractError::SignerInsufficientFunds => write!(f, "The signer does not have sufficient funds"),
            ContractError::NotEnoughAllowance => write!(f, "Not enough allowance to fund this escrow"),
            ContractError::OnlySignerCanCompleteEscrow => write!(f, "Only the signer can complete the escrow"),
            ContractError::EscrowAlreadyCompleted => write!(f, "Escrow already completed"),
            ContractError::SignerInsufficientFundsToComplete => write!(f, "The signer does not have sufficient funds to complete this escrow"),
            ContractError::OnlyServiceProviderCanCancelEscrow => write!(f, "Only the service provider can cancel the escrow"),
            ContractError::EscrowAlreadyCancelled => write!(f, "The escrow has already been cancelled"),
            ContractError::OnlySignerCanRequestRefund => write!(f, "Only the signer can request a refund"),
            ContractError::EscrowNotCancelled => write!(f, "The escrow must be cancelled to refund the amounts"),
            ContractError::NoFundsToRefund => write!(f, "No funds available to refund"),
            ContractError::ContractHasInsufficientBalance => write!(f, "The contract has no balance to repay"), 
            ContractError::EscrowNotFound => write!(f, "Escrow not found"),
            ContractError::OnlyServiceProviderCanClaimEarnings => write!(f, "Only the service provider can claim escrow earnings"),
            ContractError::EscrowNotCompleted => write!(f, "The escrow must be completed to claim earnings"),
            ContractError::EscrowBalanceNotSufficienteToSendEarnings => write!(f, "The escrow balance must be equal to the amount of earnings defined for the escrow"),
            ContractError::ContractInsufficientFunds => write!(f, "The contract does not have sufficient funds"),
            ContractError::OnlyPlatformAddressExecuteThisFunction => write!(f, "Only the plataform address should be able to execute this function"),
            ContractError::EscrowNotInitialized => write!(f, "Escrow not Initialized"),
            ContractError::OnlyServiceProviderChangeMilstoneStatus => write!(f, "Only ServiceProvider can change MilstoneStatus"),
            ContractError::NoMileStoneDefined => write!(f, "Escrow initialized without Milestone"),
            ContractError::InvalidMileStoneIndex => write!(f, "Invalid Milestone Index"),
            ContractError::OnlyClientChangeMilstoneFlag => write!(f, "Only Client Can change Milestone Flag"),
            ContractError::OnlyDisputeResolverCanExecuteThisFunction => write!(f, "Only Dispute Resolver can execute this function"),
            ContractError::EscrowAlreadyInDispute => write!(f, "Escrow already in dispute"),
            ContractError::EscrowNotInDispute => write!(f, "Escrow not in dispute"),
            ContractError::InsufficientFundsForResolution => write!(f, "Insufficient funds for resolution")
        }
    }
}