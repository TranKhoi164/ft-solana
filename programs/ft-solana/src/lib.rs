use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  metadata::{
    create_metadata_accounts_v3,
    mpl_token_metadata::types::DataV2,
    CreateMetadataAccountsV3,
    Metadata as Metaplex
  },
  token::{self, Approve, Burn, Mint, MintTo, Token, TokenAccount, mint_to, Transfer}
};

declare_id!("FNufCrMUFRhAGRYHNbq7r8YPy8cbjc6hJvsKUFD7Qhuh");
// https://ubm6evsuvwlccrmdmsftgueow65vvsdafyrewxlsahveo5b6qebq.arweave.net/oFniVlStliFFg2SLM1COt7tayGAuIktdcgHqR3Q-gQM
#[program]
pub mod ft_solana {
    use super::*;

    // Initialize the token mint
    pub fn initiate_token(_ctx: Context<InitToken>, metadata: InitTokenParams) -> Result<()> {
      let seeds = &["mint".as_bytes(), &[_ctx.bumps.mint]];
      let signer = [&seeds[..]];

      let token_data: DataV2 = DataV2 {
          name: metadata.name,
          symbol: metadata.symbol,
          uri: metadata.uri,
          seller_fee_basis_points: 0,
          creators: None,
          collection: None,
          uses: None,
      };

      let metadata_ctx = CpiContext::new_with_signer(
          _ctx.accounts.token_metadata_program.to_account_info(),
          CreateMetadataAccountsV3 {
              payer: _ctx.accounts.payer.to_account_info(),
              update_authority: _ctx.accounts.mint.to_account_info(),
              mint: _ctx.accounts.mint.to_account_info(),
              metadata: _ctx.accounts.metadata.to_account_info(),
              mint_authority: _ctx.accounts.mint.to_account_info(),
              system_program: _ctx.accounts.system_program.to_account_info(),
              rent: _ctx.accounts.rent.to_account_info(),
          },
          &signer,
      );

      create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

      msg!("Token mint created successfully.");
      Ok(())
  }


  pub fn mint_tokens(ctx: Context<MintTokens>, quantity: u64) -> Result<()> {
    let seeds = &["mint".as_bytes(), &[ctx.bumps.mint]];
    let signer = [&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &signer,
        ),
        quantity,
    )?;

    Ok(())
}
}


#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitToken<'info> {
    #[account(mut)]
    /// CHECK: UncheckedAccount
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer, //The payer covers the rent-exempt cost
        mint::decimals = params.decimals,
        mint::authority = mint, // PDA is the mint authority => only program can mint tokens
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>, //the program is accessing the Rent sysvar, which stores rent-related data like minimum balance requirements.
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> { // not the account, but a context struct that groups multiple account needed for instruction
    #[account(
        mut,
        seeds = [b"mint"], // PDA derived from "mint" seed
        bump, // Stores the bump seed
        mint::authority = mint, // The mint itself is the authority, no external signer is required to mint new tokens. Mint account can mint new tokens, but only if program sign for it (seed, bump)
    )]
    pub mint: Account<'info, Mint>,
    #[account( // normal token account (ata)
        init, 
        payer = payer, // `payer` funds the account creation
        //It means that this token account (destination) is designed to hold tokens that were created by the mint account.
        associated_token::mint = mint, // The token account is tied to the `mint`. This Account Stores Tokens from the Mint
        associated_token::authority = payer, // `payer` (wallet)  is the owner of the token account
    )] // normal account
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>, // min lamports to keep account on chain
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>, // use when working with ATA in solana
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}