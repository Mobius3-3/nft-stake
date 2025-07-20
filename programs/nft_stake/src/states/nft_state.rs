use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NftState<'info> {
  pub owner: Pubkey,
  pub mint: u8,
  pub bump: u8,
}