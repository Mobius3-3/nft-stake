use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, 
            FreezeDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata, 
        MetadataAccount
    }, 
    token::{
        approve, 
        Approve, 
        Mint, 
        Token, 
        TokenAccount
    }
};
use crate::{states::{PoolState, UserState, StakedNftState}};

#[derive(Accounts)]
pub struct Stake<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub collection_mint: Account<'info, Mint>, // for verification

  #[account(
    seeds = [
      b"metadata",
      metadata_program.key().as_ref(),
      mint.key().as_ref(),
    ],
    seeds::program = metadata_program.key(),
    bump,
    constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
    constraint = metadata.collection.as_ref().unwrap().verified == true,
  )]
  pub metadata: Account<'info, MetadataAccount>,

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
    init, 
    payer = user,
    seeds = [b"staked_nft", mint.key().as_ref(), pool_state.key().as_ref()],
    bump,
    space = 8 + StakedNftState::INIT_SPACE,
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

impl<'info> Stake<'info> {
  pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
    self.staked_nft_state.set_inner(StakedNftState {
      owner: self.user.key(),
      mint: self.mint.key(),
      stake_start_time: Clock::get()?.unix_timestamp,
      bump: bumps.staked_nft_state,
    });

    let cpi_program = self.token_program.to_account_info();
    let cpi_accounts = Approve {
      to: self.user_nft_ata.to_account_info(),
      delegate: self.staked_nft_state.to_account_info(),
      authority: self.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    approve(cpi_ctx, 1)?;

    let seeds = &[
      b"staked_nft", 
      self.mint.to_account_info().key.as_ref(), 
      self.pool_state.to_account_info().key.as_ref(), 
      &[self.staked_nft_state.bump]
    ];

    let signers_seeds = &[&seeds[..]];
    let cpi_program = &self.metadata_program.to_account_info();
    let cpi_accounts = FreezeDelegatedAccountCpiAccounts {
      delegate: &self.staked_nft_state.to_account_info(),
      token_account: &self.user_nft_ata.to_account_info(),
      edition: &self.edition.to_account_info(),
      mint: &self.mint.to_account_info(),
      token_program: &self.token_program.to_account_info(),
    };
    // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers_seeds);

    FreezeDelegatedAccountCpi::new(
      cpi_program,
      cpi_accounts,
    ).invoke_signed(signers_seeds)?;

    self.user_state.staked_amount += 1;
    
    Ok(())

  }
}