use anchor_lang::prelude::*;
use anchor_spl::token::{
    set_authority, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token,
};

///  Revoke Mint authority for the token to allow anyone to be able to mint the token.
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` mint
/// 3. `[writeable]` payer token account
/// 4. `[]` token program
/// 5. `[]` system program
///
/// Expects the following arguments  
/// 1. args: TransferTokenArgs  
///
#[derive(Accounts)]
pub struct RevokeMintAuth<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl RevokeMintAuth<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Revoke SPL token Mint authority
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn revoke_mint_auth(ctx: Context<RevokeMintAuth>) -> Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;

        Ok(())
    }
}
