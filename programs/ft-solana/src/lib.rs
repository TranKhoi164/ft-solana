use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("FNufCrMUFRhAGRYHNbq7r8YPy8cbjc6hJvsKUFD7Qhuh");

#[program]
pub mod erc20_token {
    use super::*;

    // Initialize the token mint
    pub fn initialize(
        ctx: Context<Initialize>,
        //decimals: u8,
        initial_supply: u64,
    ) -> Result<()> {
        // Mint initial supply to the initial holder
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.initial_holder.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            initial_supply,
        )?;

        Ok(())
    }

    // Returns the balance of a token account (balanceOf in ERC20)
    pub fn balance_of(ctx: Context<BalanceOf>) -> Result<()> {
        // Balance is automatically available in ctx.accounts.token_account.amount
        Ok(())
    }

    // Transfer tokens to another account
    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    // Approve another account to spend tokens (transferFrom)
    pub fn approve(ctx: Context<ApproveSpender>, amount: u64) -> Result<()> {
        token::approve(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Approve {
                    to: ctx.accounts.to.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    // Mint new tokens (only by mint authority)
    pub fn mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    // Burn tokens
    pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    // Transfer tokens on behalf of another account (transferFrom)
    pub fn transfer_from(ctx: Context<TransferFrom>, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.delegate.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
    )]
    pub mint: Account<'info, Mint>, // generating ATA?
    #[account(mut)]
    pub initial_holder: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// Accounts for balance check
#[derive(Accounts)]
pub struct BalanceOf<'info> {
    pub token_account: Account<'info, TokenAccount>,
}

// Accounts for transfer
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Accounts for approve
#[derive(Accounts)]
pub struct ApproveSpender<'info> {
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    /// CHECK: This is the delegate being approved
    pub delegate: UncheckedAccount<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Accounts for minting
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Accounts for burning
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Accounts for transferFrom
#[derive(Accounts)]
pub struct TransferFrom<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    /// CHECK: This must be the approved delegate
    pub delegate: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
