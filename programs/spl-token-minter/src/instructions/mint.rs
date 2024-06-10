use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTokenArgs {
    /// How much of the token do you want to mint
    pub amount: u64,
}

///  Mint SPL Token.
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` mint
/// 3. `[writeable]` recipient token account
/// 4. `[]` token program
/// 5. `[]` associated token program
/// 6. `[]` system program
///
/// Expects the following arguments  
/// 1. args: MintTokenArgs  
///
#[derive(Accounts)]
#[instruction(args: MintTokenArgs)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        init_if_needed,   
        payer = payer,  
        associated_token::authority = payer,
        associated_token::mint = mint,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl MintToken<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// mint SPL token
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn mint(ctx: Context<MintToken>, args: MintTokenArgs) -> Result<()> {
        let MintTokenArgs { amount } = args;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}
