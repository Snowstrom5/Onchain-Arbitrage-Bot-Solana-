use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// TODO: Replace with actual Vertigo program ID once available
pub fn vertigo_program_id() -> Pubkey {
    Pubkey::from_str("vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ").unwrap()
}
