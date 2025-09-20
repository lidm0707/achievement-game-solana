use anchor_lang::prelude::*;

declare_id!("6KK6RHywHB93cTosJSrtBS34eJmsnaZdC4DPQjk4g5HC");

#[program]
pub mod achieve_game {
    use std::str::FromStr;

    use super::*;
    const ADMIN: &str = "AKggR6oyj1amKGwBu1PVjxZTnmgog72ujp6p1f6S78o9";

    pub fn initialize(
        ctx: Context<Initialize>,
        game_id: u64,
        server_id: u64,
        provider_id: u64,
        deadline: i64, // << เพิ่มเข้ามา
        event_id: u64,
    ) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.owner.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );
        let game = &mut ctx.accounts.game;
        game.game_id = game_id;
        game.score = 0;
        game.deadline = deadline;

        // game.owner = ctx.accounts.owner.key();
        msg!("initail {} {} {}", server_id, provider_id, event_id);
        Ok(())
    }

    pub fn ongoing(ctx: Context<OnGoing>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.owner.key(),
            Pubkey::from_str(ADMIN).unwrap(),
            ErrorCode::Unauthorized
        );

        let clock = Clock::get()?; // อ่านเวลาปัจจุบันจาก slot
        let now = clock.unix_timestamp;

        require!(now <= ctx.accounts.game.deadline, ErrorCode::DeadlinePassed);

        let game = &mut ctx.accounts.game;
        game.score += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_id: u64, server_id: u64, provider_id: u64 ,event_id: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        // space = 8 + 8 +8 + 32 , // 8 bytes discriminator + 8 bytes game_id 32 pub_key
        space = 8 + 8 +8 +8 , // 8 bytes discriminator + 8 bytes game_id 32 pub_key
        seeds = [b"game",
            owner.key().as_ref(),
            &game_id.to_le_bytes().as_ref(),
            &server_id.to_le_bytes().as_ref(),
            &provider_id.to_le_bytes().as_ref(),
            &event_id.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Progress>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnGoing<'info> {
    // #[account(mut , has_one = owner)]
    #[account(mut)]
    pub game: Account<'info, Progress>,
    pub owner: Signer<'info>, // ต้องเป็น signer
}

#[account]
pub struct Progress {
    pub game_id: u64,
    pub score: u64,
    pub deadline: i64, // unix timestamp (second)

                       // pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to use this program.")]
    Unauthorized,
    #[msg("The deadline has already passed.")]
    DeadlinePassed,
}
