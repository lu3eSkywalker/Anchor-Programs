use anchor_lang::prelude::*;
// Removed: use anchor_lang::system_program;

declare_id!("7MCTi5pmxcVdJ4p4ZziJ5UHPA9pwKWEh9iYTSgLeYQQA");

#[program]
pub mod anchor_pda {
    use super::*;

    pub fn create_pda_account(ctx: Context<CreatePdaAccount>) -> Result<()> {
        msg!("PDA account created successfully");

        let pda = &mut ctx.accounts.pda_account;
        pda.owner = ctx.accounts.payer.key();
        pda.staked_amount = 0;
        pda.bump = ctx.bumps.pda_account;

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
        space = 8 + 32 + 8 + 1,
        seeds = [b"client1", payer.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub bump: u8,
}
