#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use simple_error::cpi::accounts::AlwaysFail;
use simple_error::cpi::always_fail;
use simple_error::program::SimpleError;

declare_id!("ADzUCJw47LwY1RMWDziVjtZUopAyH8RmAUY7X8ChSRoD");

#[program]
pub mod cpi_error_handle {

    use super::*;

    #[derive(Accounts)]
    pub struct TestCpiErrorHandling<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(mut)]
        pub from_token_account: Account<'info, TokenAccount>,

        #[account(mut)]
        pub to_token_account: Account<'info, TokenAccount>,

        pub token_program: Program<'info, Token>,

        pub system_program: Program<'info, System>,
    }

    pub fn test_system_cpi(ctx: Context<TestCpiErrorHandling>) -> Result<()> {
        msg!("Starting CPI error handling test");

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        // this should fail with insufficient funds
        let transfer_result = token::transfer(transfer_ctx, 2);

        msg!("CODE DIDNT REACH HERE ????");

        match transfer_result {
            Ok(_) => {
                msg!("Okkkkkkkkkkk");
            }
            Err(_err) => {
                msg!("Continuing with alternative logic...");
                // TODO: logic to update state ?
            }
        }

        Ok(())
    }

    #[derive(Accounts)]
    pub struct TestCustomCpiErrorHandling<'info> {
        pub user: Signer<'info>,
        pub simple_error_program: Program<'info, SimpleError>,
    }

    pub fn test_custom_cpi(ctx: Context<TestCustomCpiErrorHandling>) -> Result<()> {
        msg!("Starting custom CPI error handling test");

        let cpi_ctx = CpiContext::new(
            ctx.accounts.simple_error_program.to_account_info(),
            AlwaysFail {
                unused: ctx.accounts.user.to_account_info(),
            },
        );

        // This should always fail
        let cpi_result = always_fail(cpi_ctx);

        msg!("CODE DIDNT REACH HERE ????");

        match cpi_result {
            Ok(_) => {
                msg!("Unexpected success!");
            }
            Err(_err) => {
                msg!("Continuing with alternative logic...");
                // TODO: logic to update state ?
            }
        }

        Ok(())
    }
}
