use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;
use crate::instructions::create_quest::*;
use crate::instructions::make_quest::*;
use crate::state::quest_info::*;
use error::*;
declare_id!("FP3ZzVzsZopXXRTiivLqjpDSGVSuytJ5Xy4c4i4P9G7T");
const ADMIN: &str = "AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9";
use std::str::FromStr;
#[program]
pub mod quest {

    use super::*;

    pub fn create_quest(
        ctx: Context<CreateQuest>,
        quest_id: u64,
        user_id: u64,
        server_id: u64,
        provider_id: u64,
        deadline: i64,
        max_score: u64,
    ) -> Result<()> {
        let score = 0;
        ctx.accounts.quest_score.create_quest(
            quest_id,
            user_id,
            server_id,
            provider_id,
            deadline,
            score,
            max_score,
        )?;

        Ok(())
    }

    pub fn make_quest(
        ctx: Context<MakeQuest>,
        quest_id: u64,
        user_id: u64,
        server_id: u64,
        provider_id: u64,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.server_admin.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCustome::Unauthorized
        );

        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp <= ctx.accounts.quest_score.deadline,
            ErrorCustome::DeadlinePassed
        );

        ctx.accounts.quest_score.update_score()?;
        Ok(())
    }
}
