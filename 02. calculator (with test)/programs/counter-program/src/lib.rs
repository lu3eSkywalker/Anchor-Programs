use anchor_lang::prelude::*;

declare_id!("EqJhqVA7kssaFNXswwXJyrqViERu9a7ZmpGhy2duQ64u");

#[program]
pub mod counter_program {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 1;
        counter.authority = ctx.accounts.user.key(); // fix here
        Ok(())
    }

    pub fn double(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.saturating_mul(2);
        Ok(())
    }

    pub fn half(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count /= 2;
        Ok(())
    }

    pub fn add(ctx: Context<UpdateCounter>, amount: u32) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.saturating_add(amount);
        Ok(())
    }

    pub fn subtract(ctx: Context<UpdateCounter>, amount: u32) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.saturating_sub(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32)] // fix space
    pub counter: Account<'info, CounterState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, CounterState>,

    pub authority: Signer<'info>,
}

#[account]
pub struct CounterState {
    pub count: u32,
    pub authority: Pubkey, // optional: to ensure only creator can modify
}
