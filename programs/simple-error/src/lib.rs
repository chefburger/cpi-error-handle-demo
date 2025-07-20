#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("CBVN1WJ7zPCJwVtowmeQ7n5FqeXQTLtLCWprc1JVY2fM");

#[program]
pub mod simple_error {

    use super::*;

    pub fn always_fail(_ctx: Context<AlwaysFail>) -> Result<()> {
        msg!("This instruction always fails!");
        return err!(ErrorCode::AlwaysFail);
    }
}

#[derive(Accounts)]
pub struct AlwaysFail<'info> {
    /// CHECK: This is a dummy account
    pub unused: UncheckedAccount<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("This instruction is designed to always fail")]
    AlwaysFail,
}
