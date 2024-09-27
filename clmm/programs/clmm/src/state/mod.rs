use anchor_lang::prelude::*;
use crate::errors::*;

#[account]
pub struct SrAmmPair {
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub current_slot: u64,
    pub last_update_slot: u64,
    pub fee_percentage: u16,
    pub base_factor: u16,
    pub active_id: i32,
    pub bin_step: u16,
    pub protocol_fee: ProtocolFee,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ProtocolFee {
    pub amount_x: u64,
    pub amount_y: u64,
}

impl SrAmmPair {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 2 + 2 + 1;

    pub fn initialize(
        &mut self,
        bump: u8,
        active_id: i32,
        bin_step: u16,
        token_mint_x: Pubkey,
        token_mint_y: Pubkey,
        reserve_x: Pubkey,
        reserve_y: Pubkey,
        fee_percentage: u16,
        base_factor: u16,
    ) -> Result<()> {
        self.active_id = active_id;
        self.bin_step = bin_step;
        self.token_x_mint = token_mint_x;
        self.token_y_mint = token_mint_y;
        self.reserve_x = reserve_x;
        self.reserve_y = reserve_y;
        self.fee_percentage = fee_percentage;
        self.base_factor = base_factor;
        self.bump = bump;
        self.current_slot = Clock::get()?.slot;
        self.last_update_slot = self.current_slot;
        Ok(())
    }

    pub fn get_base_fee(&self) -> Result<u128> {
        Ok(u128::from(self.base_factor)
            .checked_mul(self.bin_step.into())
            .ok_or(SrAmmError::MathOverflow)?
            .checked_mul(10u128)
            .ok_or(SrAmmError::MathOverflow)?)
    }

    pub fn get_total_fee(&self) -> Result<u128> {
        let base_fee = self.get_base_fee()?;
        let total_fee = base_fee.checked_add(self.fee_percentage.into())
            .ok_or(SrAmmError::MathOverflow)?;
        Ok(std::cmp::min(total_fee, 1_000_000_000)) // Cap at 100%
    }

    pub fn compute_fee(&self, amount: u64) -> Result<u64> {
        let total_fee_rate = self.get_total_fee()?;
        let fee = u128::from(amount)
            .checked_mul(total_fee_rate)
            .ok_or(SrAmmError::MathOverflow)?
            .checked_div(1_000_000_000) // Assuming fee is in basis points
            .ok_or(SrAmmError::MathOverflow)?;
        Ok(fee.try_into().map_err(|_| SrAmmError::MathOverflow)?)
    }

    pub fn accumulate_protocol_fees(&mut self, fee_amount_x: u64, fee_amount_y: u64) -> Result<()> {
        self.protocol_fee.amount_x = self.protocol_fee.amount_x.checked_add(fee_amount_x).ok_or(SrAmmError::MathOverflow)?;
        self.protocol_fee.amount_y = self.protocol_fee.amount_y.checked_add(fee_amount_y).ok_or(SrAmmError::MathOverflow)?;
        Ok(())
    }

    pub fn withdraw_protocol_fee(&mut self, amount_x: u64, amount_y: u64) -> Result<()> {
        self.protocol_fee.amount_x = self.protocol_fee.amount_x.checked_sub(amount_x).ok_or(SrAmmError::MathOverflow)?;
        self.protocol_fee.amount_y = self.protocol_fee.amount_y.checked_sub(amount_y).ok_or(SrAmmError::MathOverflow)?;
        Ok(())
    }

    pub fn update_slot(&mut self) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        self.last_update_slot = self.current_slot;
        self.current_slot = current_slot;
        Ok(())
    }
}