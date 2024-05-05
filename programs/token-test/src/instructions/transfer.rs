use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TransferTokenArgs {
    /// How much of the token do you want to transfer
    pub amount: u64,
}

///  Transfer SPL Token.
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` mint
/// 3. `[writeable]` payer token account
/// 4. `[writeable]` recipient token account
/// 5. `[]` token program
/// 6. `[]` associated token program
/// 7. `[]` system program
///
/// Expects the following arguments  
/// 1. args: TransferTokenArgs  
///
#[derive(Accounts)]
#[instruction(args: TransferTokenArgs)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl TransferToken<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Transfer SPL token
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn transfer(ctx: Context<TransferToken>, args: TransferTokenArgs) -> Result<()> {
        let TransferTokenArgs { amount } = args;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.payer_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, 9)?;

        Ok(())
    }
}
