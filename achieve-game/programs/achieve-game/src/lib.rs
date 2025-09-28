use anchor_lang::prelude::*;

// declare_id!("6KK6RHywHB93cTosJSrtBS34eJmsnaZdC4DPQjk4g5HC");

declare_id!("EMuQqLFUEEJouvbMsGuUfkWhT5XxvNCYw1KpwQWc1vUK");
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
        msg!("👉 Initialize called with game_id={}, server_id={}, provider_id={}, event_id={}, deadline={}",
            game_id, server_id, provider_id, event_id, deadline);

        msg!("Owner signer: {}", ctx.accounts.owner.key());
        msg!("ADMIN const: {}", ADMIN);

        require_keys_eq!(
            ctx.accounts.owner.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );

        let game = &mut ctx.accounts.game;
        game.game_id = game_id;
        game.score = 0;
        game.deadline = deadline;

        msg!(
            "✅ Initialize success: PDA={}, deadline={}",
            ctx.accounts.game.key(),
            deadline
        );
        Ok(())
    }

    pub fn ongoing(ctx: Context<OnGoing>, event_id: u64) -> Result<()> {
        msg!("👉 Ongoing called with event_id={}", event_id);
        msg!("Owner signer: {}", ctx.accounts.owner.key());
        msg!("Game PDA: {}", ctx.accounts.game.key());
        msg!("Reward PDA (client): {}", ctx.accounts.reward.key());

        require_keys_eq!(
            ctx.accounts.owner.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );

        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        msg!(
            "⏰ Current time={}, game.deadline={}",
            now,
            ctx.accounts.game.deadline
        );

        require!(now <= ctx.accounts.game.deadline, ErrorCode::DeadlinePassed);

        let game = &mut ctx.accounts.game;
        game.score += 1;
        msg!("🎯 Score updated: {}", game.score);

        if game.score == 10 {
            msg!("🏆 Score reached 10, checking reward...");

            let (reward_pda, _bump) = Pubkey::find_program_address(
                &[
                    b"reward",
                    ctx.accounts.owner.key().as_ref(),
                    &event_id.to_le_bytes(),
                ],
                &ctx.accounts.reward_program.key(),
            );

            msg!("Derived reward PDA (program) = {}", reward_pda);

            require_keys_eq!(
                reward_pda,
                ctx.accounts.reward.key(),
                ErrorCode::EmptyReward
            );

            msg!("✅ Reward PDA matched, invoking reward::update_reward");

            let cpi_program = ctx.accounts.reward_program.to_account_info();
            let cpi_accounts = reward::cpi::accounts::UpdateReward {
                authority: ctx.accounts.owner.to_account_info(),
                reward: ctx.accounts.reward.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            reward::cpi::update_reward(cpi_ctx)?;
            msg!("🎉 Reward updated successfully");
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_id: u64, server_id: u64, provider_id: u64 ,event_id: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 8 + 8,
        seeds = [b"game",
            owner.key().as_ref(),
            &game_id.to_le_bytes(),
            &server_id.to_le_bytes(),
            &provider_id.to_le_bytes(),
            &event_id.to_le_bytes()],
        bump
    )]
    pub game: Account<'info, Progress>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnGoing<'info> {
    #[account(mut)]
    pub game: Account<'info, Progress>,
    pub owner: Signer<'info>,

    #[account(mut)]
    pub reward: Account<'info, reward::Reward>,
    pub reward_program: Program<'info, reward::program::Reward>,
}

#[account]
pub struct Progress {
    pub game_id: u64,
    pub score: u64,
    pub deadline: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to use this program.")]
    Unauthorized,
    #[msg("The deadline has already passed.")]
    DeadlinePassed,
    #[msg("Reward is empty")]
    EmptyReward,
}
