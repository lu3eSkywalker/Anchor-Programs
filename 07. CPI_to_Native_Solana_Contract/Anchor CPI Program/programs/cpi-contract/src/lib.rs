use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::AccountMeta;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("4krvJ1r17we6KN5gECQ9iTQeDJaKJAUjJkGkorBbH3cE");

#[program]
pub mod cpi_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let accounts = vec![AccountMeta::new(*ctx.accounts.data_account.key, true)];

        let instruction = Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts,
            data: vec![0], // InstructionType::Init
        };

        invoke(
            &instruction,
            &[
                ctx.accounts.data_account.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn double(ctx: Context<Initialize>) -> Result<()> {
        let accounts = vec![AccountMeta::new(*ctx.accounts.data_account.key, false)];

        let instruction = Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts,
            data: vec![2], // InstructionType::Double
        };

        invoke(
            &instruction,
            &[
                ctx.accounts.data_account.to_account_info(), // only this is required
            ],
        )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,
    #[account(signer)]
    pub user_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: CPI target program
    pub cpi_program: AccountInfo<'info>,
}
