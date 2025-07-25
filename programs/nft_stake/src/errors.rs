use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Freeze period has not ended")]
    FreezePeriodNotPassed,
}