
use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod flow_platform {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.client = ctx.accounts.client.key();
        escrow_account.freelancer = ctx.accounts.freelancer.key();
        escrow_account.amount = amount;
        Ok(())
    }

    pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
        let escrow_account = &ctx.accounts.escrow_account;

        **ctx.accounts.freelancer.to_account_info().try_borrow_mut_lamports()? += escrow_account.amount;
        **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= escrow_account.amount;

        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub client: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(init, payer = client, space = 8 + 32 + 32 + 8)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub client: Signer<'info>,
    pub freelancer: AccountInfo<'info>, // Freelancer account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub freelancer: AccountInfo<'info>, // Freelancer account
}
