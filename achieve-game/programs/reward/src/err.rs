use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCustome {
    #[msg("Reward is empty")]
    EmptyReward,
    #[msg("Invalid quest data")]
    InvalidQuestData,
    #[msg("Quest not completed")]
    QuestNotCompleted,
}
