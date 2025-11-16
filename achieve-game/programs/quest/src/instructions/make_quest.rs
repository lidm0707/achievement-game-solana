use crate::state::quest_info::QuestInfo;
use anchor_lang::prelude::*;
#[derive(Accounts)]
#[instruction(quest_id: u64,user_id:u64, server_id: u64, provider_id: u64 )]
pub struct MakeQuest<'info> {
    #[account(
        mut,
        seeds = [
            b"quest",
            server_admin.key().as_ref(),
            &quest_id.to_le_bytes().as_ref(),
            &user_id.to_le_bytes().as_ref(),
            &server_id.to_le_bytes().as_ref(),
            &provider_id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub quest_score: Account<'info, QuestInfo>,

    #[account(mut)]
    pub server_admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
