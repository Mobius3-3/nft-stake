use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
  pub point: u32,
  pub staked_amount: u8,
  pub bump: u8,
}