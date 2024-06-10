pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2JdPJfQSVt7Y2xEZdv1bENsLizPn3a7KxoWgsu8Caf7h");

#[program]
pub mod spl_token_minter {
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
