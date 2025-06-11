use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;

declare_id!("3ik8UYJGxRtHqbEaXbCTkWt293yG55cc93tC2gWXiMUp");

#[program]
pub mod basic_cpi_contract_2 {
    use super::*;

    pub fn solana_transfer(ctx: Context<SolanaTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let account_metas = vec![
            AccountMeta::new(from_pubkey.key(), true),
            AccountMeta::new(to_pubkey.key(), false),
        ];

        // 2 is the `SystemInstruction::Transfer` discriminator
        let instruction_discriminator: u32 = 2;

        let mut instruction_data = Vec::with_capacity(4 + 8);
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction = Instruction {
            program_id: system_program.key(),
            accounts: account_metas,
            data: instruction_data,
        };

        invoke(&instruction, &[from_pubkey, to_pubkey, system_program])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolanaTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}