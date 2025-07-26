use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventAccount {
    pub organizer: Pubkey,
    pub price: u16,
    pub tickets_total: u16,
    pub tickets_sold: u16,
    pub event_name: [u8; 48],
    pub event_address: [u8; 48],
}
