use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCustome {
    #[msg("You are not authorized to use this program.")]
    Unauthorized,
    #[msg("The deadline has already passed.")]
    DeadlinePassed,
    #[msg("Reward is empty")]
    EmptyReward,
    #[msg("Invalid reward account")]
    InvalidRewardAccount,
    #[msg("Score is not enough")]
    ScoreNotEnough,
}
