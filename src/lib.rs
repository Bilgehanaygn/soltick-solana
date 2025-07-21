use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::{ProgramResult},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use solana_system_interface::instruction::{create_account, transfer};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventAccount {
    pub organizer: Pubkey,
    pub price: u16,
    pub tickets_total: u16,
    pub tickets_sold: u16
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EventInstruction {
    CreateEvent {price: u16, tickets_total: u16},
    BuyTicket
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EventInstruction::try_from_slice(instruction_data)?;

    match instruction {
        EventInstruction::CreateEvent {
            price,
            tickets_total
        } => {
            msg!("Instruction: Create Event");
            create_event(program_id, accounts, price, tickets_total)
        }
        EventInstruction::BuyTicket => {
            msg!("Instruction: Buy Ticket");
            buy_ticket(program_id, accounts)
        }
    }
}

fn create_event(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    price: u16,
    tickets_total: u16,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let organizer_account = next_account_info(accounts_iter)?;
    let event_account = next_account_info(accounts_iter)?;
    let rent_sysvar_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if !organizer_account.is_signer {
        msg!("Organizer account must be the signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if event_account.owner != program_id {
        msg!("Event account is not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = &Rent::from_account_info(rent_sysvar_account)?;


    let account_span = borsh::to_vec(&EventAccount {
        organizer: Pubkey::default(),
        price: 0,
        tickets_total: 0,
        tickets_sold: 0
    })?.len();

    
    invoke(
        &create_account(
            organizer_account.key,
            event_account.key, 
            rent.minimum_balance(account_span),
            account_span as u64, 
            program_id), 
            &[
                organizer_account.clone(),
                event_account.clone(),
                system_program_account.clone(),
            ])?;

            let mut event_data = EventAccount::try_from_slice(&event_account.data.borrow())?;
            if event_data.organizer != Pubkey::default(){
                msg!("Event account is already initialized");
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            event_data.organizer = *organizer_account.key;
            event_data.price = price;
            event_data.tickets_total = tickets_total;
            event_data.tickets_sold = 0;

            event_data.serialize(&mut &mut event_account.data.borrow_mut()[..])?;

            msg!("Event created for organizer: {}", organizer_account.key);
            msg!("Event account address: {}", event_account.key);
            msg!("Price: {} lamports", price);
            msg!("Tickets total: {}", tickets_total);

            Ok(())
}

fn buy_ticket(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let buyer_account = next_account_info(accounts_iter)?;
    let event_account = next_account_info(accounts_iter)?;
    let organizer_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if !buyer_account.is_signer {
        msg!("Buyer account must be the signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if event_account.owner != program_id {
        msg!("Event account is not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut event_data = EventAccount::try_from_slice(&event_account.data.borrow())?;

    if event_data.organizer != *organizer_account.key {
        msg!("Incorrect organizer account provided");
        return Err(ProgramError::InvalidAccountData);
    }

    invoke(
        &transfer(buyer_account.key,
            organizer_account.key,
            event_data.price as u64), 
            &[
                buyer_account.clone(),
                organizer_account.clone(),
                system_program_account.clone()
            ]
        )?;

        event_data.tickets_sold +=1;
        event_data.serialize(&mut &mut event_account.data.borrow_mut()[..])?;

        msg!(
            "Ticket purchased successfully by {}",
            buyer_account.key
        );
        msg!(
            "Tickets remaining: {}",
            event_data.tickets_total - event_data.tickets_sold
        );

        Ok(())
}