use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::Metadata,
    token::{Mint, Token},
};
use mpl_token_metadata::{accounts::Metadata as MetadataAccount, instructions::CreateV1CpiBuilder};

use crate::constants::ADMIN_ADDRESS;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateTokenArgs {
    /// The name/title of your token.
    pub name: String,
    /// The symbol for your token. Optional, Defaults to none when not argument is passed.
    pub symbol: Option<String>,
    /// Off-chain Metadata URI string
    pub uri: String,
    /// no. of the decimals for the program.
    pub decimals: u8,
    // supply
    pub supply: u64,
}

/// Create SPL Token Mint and use MPL Metadata program to add metadata to it.
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` mint
/// 3. `[writeable]` metadata account
/// 4. `[]` metadata program
/// 5. `[]` token program
/// 6. `[]` system program`
/// 7. `[]` sysvar instruction program`
///
/// Expects the following arguments  
/// 1. args: CreateTokenArgs  
///
#[derive(Accounts)]
#[instruction(args: CreateTokenArgs)]
pub struct CreateToken<'info> {
    #[account(mut, address = ADMIN_ADDRESS)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = args.decimals,
        mint::authority = payer.key(),
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        address = MetadataAccount::find_pda(&mint.key()).0,
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub sysvar_instruction: UncheckedAccount<'info>,
}

impl CreateToken<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Create SPL token
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn create(ctx: Context<CreateToken>, args: CreateTokenArgs) -> Result<()> {
        CreateV1CpiBuilder::new(&ctx.accounts.metadata_program)
            .metadata(&ctx.accounts.metadata_account)
            .mint(&ctx.accounts.mint.to_account_info(), true)
            .authority(&ctx.accounts.payer)
            .payer(&ctx.accounts.payer)
            .update_authority(&ctx.accounts.payer, true)
            .system_program(&ctx.accounts.system_program)
            .sysvar_instructions(&ctx.accounts.sysvar_instruction.to_account_info())
            .spl_token_program(Some(&ctx.accounts.token_program))
            .token_standard(mpl_token_metadata::types::TokenStandard::Fungible)
            .seller_fee_basis_points(0) //
            .print_supply(mpl_token_metadata::types::PrintSupply::Limited(args.supply))
            .name(args.name)
            .symbol(args.symbol.unwrap_or_default())
            .uri(args.uri)
            .decimals(args.decimals)
            .invoke()?;

        Ok(())
    }
}
