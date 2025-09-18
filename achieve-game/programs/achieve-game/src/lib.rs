use anchor_lang::prelude::*;

declare_id!("6KK6RHywHB93cTosJSrtBS34eJmsnaZdC4DPQjk4g5HC");

#[program]
pub mod achieve_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, game_id: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.game_id = game_id;
        game.score = 0;
        Ok(())
    }

    pub fn ongoing(ctx: Context<OnGoing>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.score += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 +8 +8, // 8 bytes discriminator + 8 bytes game_id
        seeds = [b"game", user.key().as_ref(), game_id.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Progress>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnGoing<'info> {
    #[account(mut)]
    pub game: Account<'info, Progress>,
}

#[account]
pub struct Progress {
    pub game_id: u64,
    pub score: u64,
}
