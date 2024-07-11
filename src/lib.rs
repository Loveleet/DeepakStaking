use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TransferChecked, Mint};
use solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare your program ID
declare_id!("AXEyv4kp7Y2x8MHc5nVywSPZcNSoUaUknj8yB3JE12oE");

#[program]
pub mod token_staking {
    use super::*;

    // Initialization function for staking account
    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;

        // Ensure the staking account is not already initialized
        if staking_account.is_initialized {
            msg!("Staking account already initialized");
            return Err(ProgramError::AccountAlreadyInitialized.into());
        }

        staking_account.owner = owner;
        staking_account.token_mint = ctx.accounts.token_mint.key();
        staking_account.token_account = ctx.accounts.token_account.key();
        staking_account.is_initialized = true;
        Ok(())
    }

    // Stake functions for different amounts
    pub fn stake24_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    pub fn stake36_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    pub fn stake60_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    // Unstake functions for different amounts
    pub fn unstake30(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        private::unstake(ctx, amount)
    }

    pub fn unstake40(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        private::unstake(ctx, amount)
    }

    // Private module for internal operations
    mod private {
        use super::*;

        pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
            let staking_account = &ctx.accounts.staking_account;

            // Check if token mints match staking account
            if ctx.accounts.from.mint != staking_account.token_mint {
                msg!("Invalid token mint for the from account");
                return Err(ProgramError::InvalidArgument.into());
            }
            if ctx.accounts.to.key() != staking_account.token_account {
                msg!("Invalid token mint for the to account");
                return Err(ProgramError::InvalidArgument.into());
            }

            // Prepare CPI (Cross Program Invocation) context for token transfer
            let cpi_accounts = TransferChecked {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            // Invoke token transfer from token_interface crate
            token_interface::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

            Ok(())
        }

        pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
            let staking_account = &ctx.accounts.staking_account;

            // Check if token mints and owner match staking account
            if ctx.accounts.from.key() != staking_account.token_account {
                msg!("Invalid token mint for the from account");
                return Err(ProgramError::InvalidArgument.into());
            }
            if ctx.accounts.to.mint != staking_account.token_mint {
                msg!("Invalid token mint for the to account");
                return Err(ProgramError::InvalidArgument.into());
            }
            if ctx.accounts.owner.key() != staking_account.owner {
                msg!("Account owner mismatch");
                return Err(ProgramError::IllegalOwner.into());
            }

            // Prepare CPI (Cross Program Invocation) context for token transfer
            let cpi_accounts = TransferChecked {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            // Invoke token transfer from token_interface crate
            token_interface::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

            Ok(())
        }
    }
}

// Struct for staking account
#[account]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub is_initialized: bool,
}

// Struct for initialization context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 32 + 1)] // Allocate sufficient space
    pub staking_account: Account<'info, StakingAccount>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: AccountInfo<'info>, // Corrected to AccountInfo
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
}

// Struct for stake context
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to: InterfaceAccount<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
}

// Struct for unstake context
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(signer, address = staking_account.owner)]
    pub owner: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
}
