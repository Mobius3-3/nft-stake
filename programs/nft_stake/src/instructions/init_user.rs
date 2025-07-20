use anchor_lang::prelude::*;

use crate::states::UserState;

#[derive(Accounts)]
pub struct InitUser<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    init,
    payer = user,
    seeds = [b"user_state", user.key().as_ref()],
    bump,
    space = 8 + UserState::INIT_SPACE
  )]
  pub user_state: Account<'info, UserState>,
  pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
  pub fn init_user(&mut self, bump: &InitUserBumps) -> Result<()> {
    self.user_state.set_inner(UserState {
      point: 0,
      staked_amount: 0,
      bump: bump.user_state,
    });

    Ok(())
  }
}