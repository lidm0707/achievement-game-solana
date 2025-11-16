use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct QuestInfo {
    pub quest_id: u64,
    pub user_id: u64,
    pub server_id: u64,
    pub provider_id: u64,
    pub deadline: i64,
    pub score: u64,
    pub max_score: u64,
}

impl QuestInfo {
    pub fn create_quest(
        &mut self,
        quest_id: u64,
        user_id: u64,
        server_id: u64,
        provider_id: u64,
        deadline: i64,
        score: u64,
        max_score: u64,
    ) -> Result<()> {
        self.quest_id = quest_id;
        self.user_id = user_id;
        self.server_id = server_id;
        self.provider_id = provider_id;
        self.deadline = deadline;
        self.score = score;
        self.max_score = max_score;
        Ok(())
    }

    pub fn update_score(&mut self) -> Result<()> {
        self.score += 1;
        Ok(())
    }
}
