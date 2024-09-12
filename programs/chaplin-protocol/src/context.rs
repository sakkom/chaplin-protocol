use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is a PDA account, no need to validate its contents
    #[account(
        init,
        payer = user,
        space = 8,
        seeds = [b"protocol-vault"],
        bump
    )]
    pub protocol_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLabel<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is a PDA account, no need to validate its contents
    pub squad_key: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32,
        seeds = [b"label", squad_key.key().as_ref()],
        bump
    )]
    pub label: Account<'info, Label>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 200,
        seeds = [b"user-profile", authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConnectLabelToUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_profile: Account<'info, UserProfile>,
}

#[derive(Accounts)]
pub struct CreateFilm<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is a PDA account, no need to validate its contents
    pub collection_mint: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        space =  8 + 32 + 32 + 4 + (32 * 2) + 4 + (32 * 3),
        seeds = [b"film", collection_mint.key().as_ref()],
        bump
    )]
    pub film: Account<'info, Film>,
    pub system_program: Program<'info, System>,
}