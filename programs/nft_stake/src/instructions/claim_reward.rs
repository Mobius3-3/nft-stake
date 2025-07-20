use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::states::{UserState, PoolState};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    mut,
    seeds = [b"reward", pool_state.key().as_ref()],
    bump,
  )]
  pub reward_mint: Account<'info, Mint>,

  #[account(
    mut,
    seeds = [b"user", user.key().as_ref()],
    bump = user_state.bump,
  )]
  pub user_state: Account<'info, UserState>,

  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = reward_mint,
    associated_token::authority = user,
  )]
  pub user_reward_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [b"pool_state"],
    bump,
  )]
  pub pool_state: Account<'info, PoolState>,

  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
  pub fn claim_reward(&mut self) -> Result<()> {
    let cpi_program = self.token_program.to_account_info();
    let cpi_accounts = MintTo {
      mint: self.reward_mint.to_account_info(),
      to: self.user_reward_ata.to_account_info(), 
      authority: self.pool_state.to_account_info(),
    };
    let seeds = &[
      b"pool_state".as_ref(),
      &[self.pool_state.bump]  
    ];
    let signers_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers_seeds);

    let _ = mint_to(cpi_ctx, self.user_state.point as u64 * 10_u64.pow(self.reward_mint.decimals as u32));

    self.user_state.point = 0;

    Ok(())
  }

}

