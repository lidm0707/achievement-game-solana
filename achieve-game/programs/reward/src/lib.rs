use anchor_lang::prelude::*;

pub mod err;
pub mod instructions;
pub mod state;
use err::ErrorCustome;
use instructions::claim_reward::*;
use instructions::create_reward::*;

declare_id!("GwRMN4fNsF8MDPr5qtNH8FUbxoYXduYVrVZMCKghAuoT");

#[program]
pub mod reward {
    use quest::state::quest_info::QuestInfo;

    use super::*;

    pub fn create_reward(ctx: Context<CreateReward>, quest_id: u64, amount: u64) -> Result<()> {
        ctx.accounts.reward.create_reward(quest_id, amount)?;
        Ok(())
    }

    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        quest_id: u64,
        user_id: u64,
        server_id: u64,
        provider_id: u64,
    ) -> Result<()> {
        let reward = &mut ctx.accounts.reward;
        require!(reward.amount > 0, ErrorCustome::EmptyReward);

        // ---------------------------
        // Read Quest PDA State
        // ---------------------------
        //
        // let data = &ctx.accounts.quest_pda.try_borrow_data()?;
        // let quest_info = QuestInfo::try_deserialize(&mut &data[..])
        //     .map_err(|_| error!(ErrorCustome::InvalidQuestData))?;
        //
        //
        //
        let quest_data = &ctx.accounts.quest_pda;

        // let quest_state = QuestInfo::try_from_slice(&quest_data[..])
        //     .map_err(|_| ErrorCustome::InvalidQuestData)?;

        // ---------------------------
        // Validate quest >= max_score
        // ---------------------------
        require!(
            quest_data.score >= quest_data.max_score,
            ErrorCustome::QuestNotCompleted
        );

        reward.claim_reward()?;
        Ok(())
    }
}
