use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use crate::state::SrAmmPair;

#[derive(Accounts)]
#[instruction(
    active_id: i32,
    bin_step: u16,
    fee_percentage: u16,
    base_factor: u16
)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<SrAmmPair>(),
        seeds = [
            b"sr-amm-pair".as_ref(),
            token_x_mint.key().as_ref(),
            token_y_mint.key().as_ref(),
            &bin_step.to_le_bytes(),
        ],
        bump
    )]
    pub sr_amm_pair: Account<'info, SrAmmPair>,

    #[account(
        constraint = token_x_mint.key() != token_y_mint.key()
    )]
    pub token_x_mint: Account<'info, Mint>,
    pub token_y_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = token_x_mint,
        token::authority = sr_amm_pair,
        seeds = [
            b"reserve-x".as_ref(),
            sr_amm_pair.key().as_ref(),
        ],
        bump
    )]
    pub reserve_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = token_y_mint,
        token::authority = sr_amm_pair,
        seeds = [
            b"reserve-y".as_ref(),
            sr_amm_pair.key().as_ref(),
        ],
        bump
    )]
    pub reserve_y: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}



pub fn initialize(ctx: Context<Initialize>, active_id: i32, bin_step: u16, fee_percentage: u16, base_factor: u16) -> Result<()> {
    let sr_amm_pair = &mut ctx.accounts.sr_amm_pair;

    sr_amm_pair.initialize(
        ctx.bumps.sr_amm_pair,
        active_id,
        bin_step,
        ctx.accounts.token_x_mint.key(),
        ctx.accounts.token_y_mint.key(),
        ctx.accounts.reserve_x.key(),
        ctx.accounts.reserve_y.key(),
        fee_percentage,
        base_factor,
    )?;

    Ok(())
}