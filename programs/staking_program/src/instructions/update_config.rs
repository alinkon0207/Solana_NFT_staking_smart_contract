use crate::{constants::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangePoolSetting<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [RS_PREFIX.as_bytes()],
        bump,
        has_one = admin,
        constraint = pool_account.is_initialized == true,
    )]
    pub pool_account: Account<'info, PoolConfig>,
}

pub fn handle(
    ctx: Context<ChangePoolSetting>,
    reward_per_week: u16,
    paused: bool,
) -> Result<()> {
    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.paused = paused; // initial status is paused
    pool_account.last_update_time = Clock::get()?.unix_timestamp;
    pool_account.reward_per_week = reward_per_week;
    Ok(())
}
