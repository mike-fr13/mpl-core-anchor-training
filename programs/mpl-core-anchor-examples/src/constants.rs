use anchor_lang::prelude::*;

#[constant]
pub const JACKPOT_SEED: &str = "jackpot_vault";
pub const JACKPOT_FEES: u64 = 1000000;
pub const JACKPOT_MAX_TICKETS: u32 = 2;

// JACKPOT_WINNING_TICKET_ID must be < JACKPOT_MAX_TICKETS if you want winners :)
pub const JACKPOT_WINNING_TICKET_ID: u32 = 0;
