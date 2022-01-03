use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[error]
pub enum ErrorCode {
    #[msg("Wallet withdrawn from is not owned by the owner")]
    WalletToWithdrawFromInvalid
}

#[program]
pub mod payment {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct State {
    //primary key that derives other accounts
    idx: u64,

    //Address of the receiver
    receiver: Pubkey,

    //Address of the sender
    sender: Pubkey,

    //Amount sent to the receiver
    amount: u64,

}

#[derive(Accounts)]
pub struct Initialize {}