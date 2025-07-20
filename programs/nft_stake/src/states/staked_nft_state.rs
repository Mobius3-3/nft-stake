use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakedNftState {
  pub owner: Pubkey,
  pub mint: Pubkey,
  pub stake_start_time: i64,
  pub bump: u8,
}