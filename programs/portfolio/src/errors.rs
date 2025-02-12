use anchor_lang::prelude::*;

#[error_code]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum PortfolioError {
    #[msg("Signer not authorized.")]
    Unauthorized,
    #[msg("Accounts not provided.")]
    AccountsNotProvided,
    #[msg("Account page is full.")]
    PageFull,
    #[msg("P-NTDP-01")]
    DepositsPaused,
    #[msg("P-BANA-01")]
    AccountBanned,
    #[msg("Portofolio is paused.")]
    ProgramPaused,
    #[msg("Portfolio must be paused.")]
    ProgramNotPaused,
    #[msg("P-PTNS-02")]
    UnsupportedTransaction,
    #[msg("P-ZETD-01")]
    ZeroTokenQuantity,
    #[msg("LZ-QUOTE-ERROR")]
    LzQuoteError,
}

#[error_code]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum BannedAccountsError {
    #[msg("Account is already banned.")]
    AccountAlreadyBanned,
    #[msg("Account is not banned.")]
    AccountNotBanned,
}

#[error_code]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum TokenListError {
    #[msg("Token is already added.")]
    TokenAlreadyAdded,
    #[msg("Token is not found.")]
    TokenNotFound,
    #[msg("TokenList is full.")]
    TokenListFull,
    #[msg("Only SOL can be added as native token.")]
    InvalidNativeToken,
}

#[error_code]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum DepositError {
    #[msg("P-NETD-01")]
    NotEnoughSplTokenBalance,
    #[msg("Not enough SOL balance.")]
    NotEnoughNativeBalance,
    #[msg("Invalid vault owner.")]
    InvalidVaultOwner,
    #[msg("Token not supported.")]
    TokenNotSupported,
    #[msg("Token not found.")]
    TokenNotFound,
    #[msg("Invalid mint.")]
    InvalidMint,
    #[msg("P-OODT-01")]
    InvalidTokenOwner,
    #[msg("Invalid destination owner.")] // Add new error code?
    InvalidDestinationOwner,
    #[msg("P-NDNS-01")]
    NativeDepositNotAllowed,
    #[msg("Banned account.")]
    BannedAccount,
}

#[error_code]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum WithdrawError {
    #[msg("Program does not have enough balance.")]
    NotEnoughNativeBalance,
    #[msg("Invalid token symbol.")]
    InvalidTokenSymbol,
}
