use anchor_lang::prelude::*;

#[error_code]
pub enum BonkPawsError {
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Swap IX not found")]
    MissingSwapIx,
    #[msg("Finalize IX not found")]
    MissingFinalizeIx, 
    #[msg("Donate IX not found")]
    MissingDonateIx,
    #[msg("Invalid Program ID")]
    ProgramMismatch,
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Invalid number of routes")]
    InvalidRoute,
    #[msg("Invalid slippage")]
    InvalidSlippage,
    #[msg("Invalid Solana amount")]
    InvalidSolanaAmount,
    #[msg("Invalid BONK mint address")]
    InvalidBonkMint,
    #[msg("Invalid BONK account")]
    InvalidBonkAccount,
    #[msg("Invalid BONK ATA")]
    InvalidBonkATA,
    #[msg("Invalid wSOL mint address")]
    InvalidwSolMint,
    #[msg("Invalid wSOL ATA")]
    InvalidwSolATA,
    #[msg("Invalid wSOL account")]
    InvalidwSolAccount,
    #[msg("Invalid wSOL balance")]
    InvalidwSolBalance,
    #[msg("Invalid charity address")]
    InvalidCharityAddress,
    #[msg("Invalid charity Id")]
    InvalidCharityId,
    #[msg("Invalid lamports balance")]
    InvalidLamportsBalance,
    #[msg("Invalid instruction index")]
    InvalidInstructionIndex,
    #[msg("Signature header mismatch")]
    SignatureHeaderMismatch,
    #[msg("Signature authority mismatch")]
    SignatureAuthorityMismatch,

    #[msg("Not enough SOL Donated to Match")]
    NotMatchingDonation,
    #[msg("Invalid Match Key")]
    InvalidMatchKey,
}