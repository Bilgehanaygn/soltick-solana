use soltick_solana::state::EventAccount;
use solana_program::pubkey::Pubkey;

fn main() {
    let account_space = borsh::to_vec(&EventAccount {
        organizer: Pubkey::default(),
        price: 0,
        tickets_total: 0,
        tickets_sold: 0,
        event_name: [0u8; 48],
        event_address: [0u8; 48],
    }).unwrap().len();

    println!("EventAccount size: {}", account_space);
}
