use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sr_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, active_id: i32, bin_step: u16, fee_percentage: u16, base_factor: u16) -> Result<()> {
        instructions::initialize::initialize(ctx, active_id, bin_step, fee_percentage, base_factor)
    }

    // pub fn deposit(ctx: Context<Deposit>, amount_x: u64, amount_y: u64, min_lp: u64) -> Result<()> {
    //     instructions::deposit::deposit(ctx, amount_x, amount_y, min_lp)
    // }
    //
    // pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
    //     instructions::swap::swap(ctx, amount_in, minimum_amount_out)
    // }
}