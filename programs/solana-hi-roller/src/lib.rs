use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

use chainlink_solana as chainlink;
use Result;

declare_id!("5cvqhGJwpJGHYgLiDoSkoDYVJX7pE1wCg8xg5e5ZvkPg");




#[program]
pub mod roll_dice {
    use super::*;
    pub fn execute(ctx: Context<Execute>) -> ProgramResult  {
        let round = chainlink::latest_round_data(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info(),
        )?;

        let description = chainlink::description(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info(),
        )?;

        // Set the account value
        let dice: &mut Account<Dice> = &mut ctx.accounts.dice;
        dice.value= (round.answer % 6);

        // Print dice value to console, and where it was derived from
        msg!("Dice rolled: {}. This random number was derived from {} price", round.answer % 6, description);
        Ok(())
    }

    pub fn player_win(ctx:Context<Execute>) -> ProgramResult{
        let players = &mut ctx.accounts.player_match;
        let dice_value =  &mut ctx.accounts.dice.value;
        let value_1 = players.player_1_guess - *dice_value;
        let value_2 = players.player_2_guess - *dice_value;
        if(value_1).abs() > (value_2).abs(){
            msg!("player 1 wins");
            players.player_1_score += 1
        }
        else if (value_2).abs() > (value_1).abs(){
            msg!("player 2 wins");
            players.player_2_score += 1
        }
        else{
            msg!("No one wins in this round")
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer = user, space = 100)]
    pub dice: Account<'info, Dice>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub player_match:Account<'info,Player>,
    pub chainlink_feed: AccountInfo<'info>,
    pub chainlink_program: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Player{
    player_1_score:u64,
    player_2_score:u64,
    player_1_guess:i128,
    player_2_guess:i128,

}

#[account]
pub struct Dice {
    pub value: i128,
}

