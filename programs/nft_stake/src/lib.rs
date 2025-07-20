#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("ELQaJEYd8f8CRALo3nstHg1pXbFxWKXH8oE9nCrYsGtq");

pub mod instructions;
pub mod states;
pub mod errors;

pub use instructions::*;

#[program]
pub mod nft_stake {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, point_per_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init_pool(
            point_per_stake,
            freeze_period,
            &ctx.bumps,
        )
    }
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }


    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<ClaimReward>) -> Result<()> {
        ctx.accounts.claim_reward()
    }
}

