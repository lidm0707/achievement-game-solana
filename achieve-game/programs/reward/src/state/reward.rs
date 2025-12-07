use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(InitSpace)]
pub struct Reward {
    pub quest_id: u64,
    pub amount: u64,
}

impl Reward {
    pub fn create_reward(&mut self, quest_id: u64, amount: u64) -> Result<()> {
        self.quest_id = quest_id;
        self.amount = amount;
        Ok(())
    }

    pub fn claim_reward(&mut self) -> Result<()> {
        self.amount -= 1;
        Ok(())
    }
}
