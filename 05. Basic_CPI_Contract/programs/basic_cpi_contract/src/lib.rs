use anchor_lang::prelude::*;

declare_id!("DgBsfhxc7VcJxHKcsgviuUjQsSzjukofpp5xZKX1FHmF");

#[program]
pub mod basic_cpi_contract {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn solana_transfer(ctx: Context<SolanaTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolanaTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
