use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount,
        Metadata
    }, 
    token::{
        Mint, 
        Token, 
        TokenAccount
    }
};
use crate::{errors::StakeError, states::{PoolState, UserState, StakedNftState}};

#[derive(Accounts)]
pub struct Unstake<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  pub mint: Account<'info, Mint>,

  #[account(
    seeds = [
        b"metadata",
        metadata_program.key().as_ref(),
        mint.key().as_ref(),
        b"edition"
    ],
    seeds::program = metadata_program.key(),
    bump,
  )]
  pub edition: Account<'info, MasterEditionAccount>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = user,
  )]
  pub user_nft_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [b"staked_nft", mint.key().as_ref(), pool_state.key().as_ref()],
    bump = staked_nft_state.bump,
  )]
  pub staked_nft_state: Account<'info, StakedNftState>,

  #[account(
    seeds = [b"pool_state"],
    bump = pool_state.bump,
  )]
  pub pool_state: Account<'info, PoolState>,

  #[account(
    mut,
    seeds = [b"user_state", user.key().as_ref()],
    bump = user_state.bump,
  )]
  pub user_state: Account<'info, UserState>,

  pub metadata_program: Program<'info, Metadata>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
  pub fn unstake(&mut self) -> Result<()> {
    let time_elapsed = (( Clock::get()?.unix_timestamp - self.staked_nft_state.stake_start_time ) / 86400) as u32; // in case overflow, should count as number of days

    require!(time_elapsed >= self.pool_state.freeze_period, StakeError::FreezePeriodNotPassed); 
    self.user_state.point += time_elapsed * self.pool_state.point_per_stake as u32; 

    let cpi_program = &self.metadata_program.to_account_info();
    let cpi_accounts = ThawDelegatedAccountCpiAccounts {
      delegate: &self.staked_nft_state.to_account_info(),
      token_account: &self.user_nft_ata.to_account_info(),
      edition: &self.edition.to_account_info(),
      mint: &self.mint.to_account_info(),
      token_program: &self.metadata_program.to_account_info(),
    };
    let seeds = &[
      b"staked_nft", 
      self.mint.to_account_info().key.as_ref(), 
      self.pool_state.to_account_info().key.as_ref(), 
      &[self.staked_nft_state.bump]
    ];
    let signers_seeds = &[&seeds[..]];

    // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers_seeds);

    ThawDelegatedAccountCpi::new(
      cpi_program,
      cpi_accounts
    ).invoke_signed(signers_seeds)?;

    self.user_state.staked_amount -= 1;
    
    Ok(())
  }
}
