use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EventInstruction {
    CreateEvent { price: u16, tickets_total: u16, event_name: [u8; 48], event_address: [u8; 48] },
    BuyTicket,
}