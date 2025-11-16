use anchor_lang::prelude::*;
use quest::{program::Quest, state::quest_info::QuestInfo};

use crate::state::reward::Reward;

#[derive(Accounts)]
#[instruction(quest_id: u64, user_id: u64, server_id: u64, provider_id: u64)]
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = [
            b"reward",
            quest_id.to_le_bytes().as_ref()],
        bump)]
    pub reward: Account<'info, Reward>,

    #[account(mut)]
    pub server_admin: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: Quest PDA from Quest program — but now specify full seeds & type
    #[account(
        owner = quest_program.key()
    )]
    pub quest_pda: Account<'info, QuestInfo>,

    /// CHECK: Quest program ID (NOT a PDA)
    pub quest_program: Program<'info, Quest>,
}

//mut,
// seeds = [
//     b"quest",
//     server_admin.key().as_ref(),
//     &quest_id.to_le_bytes().as_ref(),
//     &user_id.to_le_bytes().as_ref(),
//     &server_id.to_le_bytes().as_ref(),
//     &provider_id.to_le_bytes().as_ref(),
// ],
// bump,
