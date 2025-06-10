use anchor_lang::prelude::*;

declare_id!("GT7xXSyZwh1TxKyid8fGepiLMV8wvbmgTVaHuFFa18hw");

#[program]
pub mod string_on_pda {
    use super::*;

    pub fn create_pda_account(ctx: Context<CreatePdaAccount>, string: String) -> Result<()> {
        msg!("PDA account created successfully");

        let pda = &mut ctx.accounts.pda_account;
        pda.owner = ctx.accounts.payer.key();
        pda.string_on_blockchain = string;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePdaAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 280,
        seeds = [b"string1", payer.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, StringAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct StringAccount {
    pub owner: Pubkey,
    pub string_on_blockchain: String
}