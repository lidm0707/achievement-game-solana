use anchor_lang::prelude::*;

declare_id!("7uXYiv4Sm979vBWFBe9MzgiyFDX3Z7DwPL5dXEqcYpLx");

#[program]
pub mod achieve_game {
    use super::*;
    use std::str::FromStr;

    const ADMIN: &str = "AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9";

    pub fn initialize(
        ctx: Context<Initialize>,
        game_id: u64,
        server_id: u64,
        provider_id: u64,
        deadline: i64,
        event_id: u64,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.admin.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );

        let game = &mut ctx.accounts.game;
        game.game_id = game_id;
        game.score = 0;
        game.deadline = deadline;

        Ok(())
    }

    pub fn ongoing(ctx: Context<OnGoing>, event_id: u64) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.admin.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );

        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp <= ctx.accounts.game.deadline,
            ErrorCode::DeadlinePassed
        );

        let game = &mut ctx.accounts.game;
        game.score += 1;

        if game.score == 10 {
            // ✅ derive reward PDA ตาม seed ฝั่ง reward program
            let (reward_pda, _bump) = Pubkey::find_program_address(
                &[
                    b"reward",
                    ctx.accounts.admin.key().as_ref(),
                    &event_id.to_le_bytes(),
                ],
                &ctx.accounts.reward_program.key(),
            );

            // ✅ เช็คว่า PDA ที่ derive ได้ ต้องตรงกับ account ที่ส่งมา
            require_keys_eq!(
                reward_pda,
                ctx.accounts.reward.key(),
                ErrorCode::InvalidRewardAccount
            );

            // ✅ เตรียม CPI
            let cpi_program = ctx.accounts.reward_program.to_account_info();
            let cpi_accounts = reward_achie::cpi::accounts::UpdateReward {
                authority: ctx.accounts.admin.to_account_info(),
                reward: ctx.accounts.reward.to_account_info(),
            };

            // ✅ ใส่ seeds ของ admin (ถ้า reward program ใช้ check authority)
            // let seeds = &[b"game", ctx.accounts.admin.key.as_ref()];
            // let signer_seeds = &[&seeds[..]]; // optional ถ้า reward_achie ต้องการ signer PDA

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            reward_achie::cpi::update_reward(cpi_ctx, event_id)?;

            // ✅ mark clear event
            game.clear_event = true;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_id: u64, server_id: u64, provider_id: u64 , deadline: i64,event_id: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 8 + 8 +1 +1,
        seeds = [
            b"game",
            admin.key().as_ref(),
            &game_id.to_le_bytes().as_ref(),
            &server_id.to_le_bytes().as_ref(),
            &provider_id.to_le_bytes().as_ref(),
            &event_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub game: Account<'info, Progress>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnGoing<'info> {
    #[account(mut)]
    pub game: Account<'info, Progress>,
    pub admin: Signer<'info>,

    #[account(mut)]
    pub reward: Account<'info, reward_achie::Reward>,
    pub reward_program: Program<'info, reward_achie::program::RewardAchie>,
}

#[account]
pub struct Progress {
    pub game_id: u64,
    pub score: u64,
    pub deadline: i64,
    pub clear_event: bool,
    pub minted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to use this program.")]
    Unauthorized,
    #[msg("The deadline has already passed.")]
    DeadlinePassed,
    #[msg("Reward is empty")]
    EmptyReward,
    #[msg("Invalid reward account")]
    InvalidRewardAccount,
}
