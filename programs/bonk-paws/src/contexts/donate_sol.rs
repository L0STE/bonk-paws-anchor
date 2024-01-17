use anchor_lang::{
    prelude::*, 
    solana_program::sysvar::{
        self, 
        instructions::{
            load_current_index_checked, 
            load_instruction_at_checked
        }},
    system_program::{Transfer, transfer},
};

use crate::{
    constants::*,
    errors::BonkPawsError,
    state::{DonationState, MatchDonation}
};

#[derive(Accounts)]
#[instruction(_seed: u64)]
pub struct DonateSol<'info> {
    #[account(mut)]
    donor: Signer<'info>,
    charity: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = donor,
        seeds = [b"donation_state"],
        bump,    
        space = DonationState::INIT_SPACE
    )]
    donation_state: Account<'info, DonationState>,
    #[account(
        init,
        payer = donor,
        seeds = [b"match_donation", _seed.to_le_bytes().as_ref()],
        bump,    
        space = MatchDonation::INIT_SPACE
    )]
    match_donation_state: Option<Account<'info, MatchDonation>>,

    #[account(address = sysvar::instructions::ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
    system_program: Program<'info, System>
}

impl<'info> DonateSol<'info> {        
    pub fn donate_sol(&mut self, _seed: u64, sol_donation: u64, id: u64) -> Result<()> {
        
        /* Send the SOL to the charity address and if we need to match create an istance where we Donate */

        // We check that the MatchDonation State is initialized only when the threshold is met
        require!(sol_donation < MIN_MATCH_THRESHOLD && self.match_donation_state.is_some(), BonkPawsError::NotMatchingDonation); // Double check with test

        let transfer_accounts = Transfer {
            from: self.donor.to_account_info(),
            to: self.charity.to_account_info(),
        };
        let transfer_cpi = CpiContext::new(self.system_program.to_account_info(), transfer_accounts);

        transfer(transfer_cpi, sol_donation)?;

        // If we have to match later we need to create the MatchDonation State
        if self.match_donation_state.is_some() {
            self.match_donation_state.as_mut().unwrap().set_inner(           
                MatchDonation {
                    id,
                    amount: sol_donation,
                    seed: _seed,
                }
            );
        }

        /* 
        
            Instruction Introspection

            This is the primary means by which we secure our program,
            enforce atomicity while making a great UX for our users.

        */

        let ixs = self.instructions.to_account_info();

        /*

            Disable CPIs
            
            Although we have taken numerous measures to secure this program,
            we can kill CPI to close off even more attack vectors as our 
            current use case doesn't need it.

        */

        let current_index = load_current_index_checked(&ixs)? as usize;
        require_gte!(current_index, 1, BonkPawsError::InvalidInstructionIndex);
        let current_ix = load_instruction_at_checked(current_index, &ixs)?;
        require!(crate::check_id(&current_ix.program_id), BonkPawsError::ProgramMismatch);

        /*
        
            Make sure previous IX is an ed25519 signature verifying the donation address

        */
        
        // Check program ID is instructions sysvar
        let signature_ix = load_instruction_at_checked(current_index-1, &ixs)?;
        require!(sysvar::instructions::check_id(&signature_ix.program_id), BonkPawsError::ProgramMismatch);

        // Ensure a strict instruction header format: 
        require!([0x01, 0x00, 0x30, 0x00, 0xff, 0xff, 0x10, 0x00, 0xff, 0xff, 0x70, 0x00, 0x28, 0x00, 0xff, 0xff].eq(&signature_ix.data[0..16]), BonkPawsError::SignatureHeaderMismatch);

        // Ensure signing authority is correct
        require!(signing_authority::ID.to_bytes().eq(&signature_ix.data[16..48]), BonkPawsError::SignatureAuthorityMismatch);

        // Get the charity ID for later verification
        // The following fetches the charity key for later varification
        let mut charity_key_data: [u8;32] = [0u8;32]; 
        charity_key_data.copy_from_slice(&signature_ix.data[0x70..0x90]);
        let charity_key = Pubkey::from(charity_key_data);

        // Ensure that the Transfer is going to the charity address
        require_keys_eq!(self.charity.key(), charity_key, BonkPawsError::InvalidCharityAddress);


        //The following fetches the charity ID for later varification
        let mut charity_id_data: [u8;8] = [0u8;8]; 
        charity_id_data.copy_from_slice(&signature_ix.data[0x90..0x98]);
        let charity_id = u64::from_le_bytes(charity_id_data);

        // Ensure that the Transfer is going to the ID passed in
        require_eq!(id, charity_id, BonkPawsError::InvalidCharityId);
        
        Ok(())
    }
}
