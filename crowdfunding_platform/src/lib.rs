use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignAccount {

  pub campaign_owner: Pubkey,
  pub campaign_amount: u64,
  pub campaign_description: String,
  pub campaign_fulfilled: u64,

}

entrypoint!(process_instruction);


pub fn process_instruction (
  program_id : &Pubkey,
  accounts : &[AccountInfo],
  data :&[u8],
) -> ProgramResult{
  let iterable_accounts = &mut accounts.iter();
  let campaign_account = next_account_info(iterable_accounts)?;
  let (instruction_byte, all_other_bytes) = data.split_first().unwrap();
  let amount = all_other_bytes.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).unwrap();
  let description = String::from_utf8(all_other_bytes[9..].to_vec()).unwrap();

  if *instruction_byte == 0{
    // create campaign
    let campaign_owner_account = next_account_info(iterable_accounts)?;
    let mut campaign_account_data= CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
    campaign_account_data.campaign_owner = *campaign_owner_account.owner;
    campaign_account_data.campaign_amount = amount;
    campaign_account_data.campaign_description = description;
    campaign_account_data.campaign_fulfilled = 0;
    campaign_account_data.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;
  }

  else if *instruction_byte == 1{
    // fund a campaign
    let mut campaign_account_data = CampaignAccount::try_from_slice(&campaign_account.data.borrow())?;
    msg!("{}",campaign_account_data.campaign_amount - campaign_account_data.campaign_fulfilled);
  }

  else if *instruction_byte == 2{
  // get how much funds are left to reach the requested amount
  }

  else if *instruction_byte == 3{
  // withdraw all collected funds and close campaign
  }

  Ok(())
}
