
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::states::PoolState;

#[derive(Accounts)]
pub struct InitPool<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,

  #[account(
    init,
    payer = admin,
    seeds = [b"reward", pool_state.key().as_ref()],
    bump,
    mint::authority = pool_state,
    mint::decimals = 6,
  )]
  pub reward_mint: Account<'info, Mint>,

  #[account(
    init,
    payer = admin,
    seeds = [b"pool_state"],
    bump,
    space = 8 + PoolState::INIT_SPACE,
  )]
  pub pool_state: Account<'info, PoolState>,

  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl<'info> InitPool<'info> {
  pub fn init_pool(&mut self, point_per_stake: u8, freeze_period: u32, bumps: &InitPoolBumps) -> Result<()> {
    self.pool_state.set_inner( PoolState {
      point_per_stake,
      freeze_period,
      bump_reward_mint: bumps.reward_mint,
      bump: bumps.pool_state,
    });

    Ok(())

  }
}