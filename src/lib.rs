use borsh::BorshDeserialize;
use pinocchio::{
    AccountView, Address, ProgramResult, entrypoint,
    error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;
use solana_program::program_pack::Pack;
use solana_program_log::log;
use spl_token::state::Mint;

#[derive(BorshDeserialize, Debug)]
pub struct CreateTokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

const METADATA_POINTER_SIZE: usize = 4 + 32 + 32;
const METADATA_EXTENSION_BASE_SIZE: usize = 4 + 32 + 32 + 4 + 4 + 4 + 4;
const EXTENSIONS_PADDING_AND_OFFSET: usize = 84;

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let [
        mint_account,
        mint_authority,
        payer,
        token_program,
        _system_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let args = CreateTokenArgs::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let extension_size = METADATA_POINTER_SIZE
        + METADATA_EXTENSION_BASE_SIZE
        + args.name.len()
        + args.symbol.len()
        + args.uri.len();

    let toal_mint_size = Mint::LEN + EXTENSIONS_PADDING_AND_OFFSET + extension_size;

    let rent = Rent::get()?;

    //create the account for the Mint
    CreateAccount {
        from: payer,
        to: mint_account,
        owner: token_program,
        lamports: rent.try_minimum_balance(Mint::LEN),
        space: Mint::LEN as u64,
    }
    .invoke()?;

    Ok(())
}
