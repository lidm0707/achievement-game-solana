use anchor_lang::prelude::*;

use crate::state::reward::Reward;

#[derive(Accounts)]
#[instruction(quest_id: u64)]
pub struct CreateReward<'info> {
    #[account(init, payer = server_admin,
        seeds = [
            b"reward",
            quest_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Reward::INIT_SPACE)]
    pub reward: Account<'info, Reward>,

    #[account(mut)]
    pub server_admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
