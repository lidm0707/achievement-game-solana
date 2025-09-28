use anchor_lang::prelude::*;

declare_id!("22gpiE17bYgxgbyYW9DMF3BZwQDYggbM4Qa7kNzfjgJH");

#[program]
pub mod reward {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, event_id: u64, amount: u64) -> Result<()> {
        let reward = &mut ctx.accounts.reward;
        reward.id = event_id; // อยากให้ Id ไม่ซ้ำกัน
        reward.amount = amount;
        Ok(())
    }

    pub fn update_reward(ctx: Context<UpdateReward>) -> Result<()> {
        let reward = &mut ctx.accounts.reward;
        require!(reward.amount < 1, ErrorCode::EmptyReward);
        reward.amount -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + 8 + 8 ,
        seeds = [b"reward",authority.key().as_ref(), &event_id.to_le_bytes().as_ref()], bump)]
    pub reward: Account<'info, Reward>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct UpdateReward<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut, // ✅ ใช้ mut แทน init
        seeds = [b"reward", authority.key().as_ref(), &event_id.to_le_bytes()],
        bump
    )]
    pub reward: Account<'info, Reward>,
}

#[account]
#[derive(Default)]
pub struct Reward {
    pub id: u64,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Reward is empty")]
    EmptyReward,
}
