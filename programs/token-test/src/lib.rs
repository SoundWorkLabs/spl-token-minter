pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CkrrcbKYHcBj8ozt79rVtKcrQ6iChsXWCraUPe6khynz");

#[program]
pub mod token_test {
    use super::*;

    pub fn create(ctx: Context<CreateToken>, args: CreateTokenArgs) -> Result<()> {
        CreateToken::create(ctx, args)
    }

    pub fn mint(ctx: Context<MintToken>, args: MintTokenArgs) -> Result<()> {
        MintToken::mint(ctx, args)
    }

    pub fn transfer(ctx: Context<TransferToken>, args: TransferTokenArgs) -> Result<()> {
        TransferToken::transfer(ctx, args)
    }

    pub fn revoke_mint_auth(ctx: Context<RevokeMintAuth>) -> Result<()> {
        RevokeMintAuth::revoke_mint_auth(ctx)
    }
}
