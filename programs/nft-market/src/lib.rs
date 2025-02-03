use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use mpl_token_metadata::{instructions as mpl_instruction, ID as TOKEN_METADATA_PROGRAM_ID};
use solana_program::program::invoke_signed;

declare_id!("oA1FPmR6KgkAeJ5QsxTgPKzAvzPeUCyF6fNESucurMP");

#[program]
pub mod nft_marketplace {
    use super::*;

    /// Function to mint an NFT
    pub fn mint_nft(
        ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        let metadata_accounts = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ];

        let metadata_instruction = mpl_instruction::create_metadata_accounts_v3(
            TOKEN_METADATA_PROGRAM_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
            None,
            1,
            true,
            true,
            None,
            None,
            None,
        );

        invoke_signed(
            &metadata_instruction,
            metadata_accounts.as_slice(),
            &[],
        )?;

        Ok(())
    }

    /// Function to transfer an NFT to another wallet
    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }

    /// Function to update NFT metadata
    pub fn update_nft_metadata(
        ctx: Context<UpdateMetadata>,
        new_uri: String,
    ) -> Result<()> {
        let update_metadata_instruction = mpl_instruction::update_metadata_accounts_v2(
            TOKEN_METADATA_PROGRAM_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.owner.key(),
            None,
            Some(new_uri),
            None,
            None,
        );

        invoke_signed(
            &update_metadata_instruction,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(init, payer = payer, mint::decimals = 0, mint::authority = payer)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = payer, associated_token::mint = mint, associated_token::authority = payer)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    #[account(address = anchor_spl::associated_token::ID)]
pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferNft<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

