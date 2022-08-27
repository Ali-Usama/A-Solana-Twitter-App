use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("6FmHN6wiNUZrTmZCFPBHnhg8YSj9KeSekzc1AcsgAKXt");

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer=author, space=Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    // #[account(address=system_program::ID)]   to ensure that it's the official system account
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String
}


const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50*4;  // for 50 characters
const MAX_CONTENT_LENGTH: usize = 280*4;

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet = &mut ctx.accounts.tweet;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 50 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;   // dereferencing the public key
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        Ok(())
    }
}

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH
                    + TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX
                    + MAX_TOPIC_LENGTH + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;


}

// #[derive(Accounts)]
// pub struct Initialize {}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum")]
    TopicTooLong,
    #[msg("The provided topic should be 280 characters long maximum")]
    ContentTooLong,
}