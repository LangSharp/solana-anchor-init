use anchor_lang::prelude::*;

declare_id!("CyW9Ck7HrNQryT1mGobveET4BT1At7SGu4mpQWWqabAb");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let  counter = &mut ctx.accounts.my_counter;
        counter.count = 0;
        counter.owner = ctx.accounts.user.key();
        msg!("Contador created for: {}", counter.owner);
        Ok(())
    }

    pub fn update(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.my_counter;
        counter.count += 1;
        msg!("The value counter is: {}", counter.count);
        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        msg!("Cuenta Cerrada, SOL recuperado");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 48 )]
    pub my_counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info > {
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub my_counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = receiver, has_one = owner @ ErrorCode::Unauthorized)]
    pub my_counter: Account<'info, Counter>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No tienes permiso de modificar este contador")]
    Unauthorized
}

#[account]
pub struct Counter {
    pub count: u64,
    pub owner: Pubkey,
}
