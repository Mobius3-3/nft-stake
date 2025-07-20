use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolState {
  pub point_per_stake: u8,
  pub freeze_period: u32,
  pub bump_reward_mint: u8,
  pub bump: u8,
}