use crate::{context::*, state::Actor};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    Ok(())
}

pub fn create_label(
    ctx: Context<CreateLabel>,
    squad_key: Pubkey,
    bubblegum_tree: Pubkey,
) -> Result<()> {
    let label = &mut ctx.accounts.label;

    label.squad_key = squad_key;
    label.bubblegum_tree = bubblegum_tree;

    Ok(())
}

pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.name = name;
    user_profile.authority = *ctx.accounts.authority.key;

    Ok(())
}

pub fn connect_label_to_user(ctx: Context<ConnectLabelToUser>, label: Pubkey) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.label.push(label.key());

    Ok(())
}

pub fn create_film(ctx: Context<CreateFilm>, collection_mint: Pubkey, label: Pubkey, actor: Actor) -> Result<()> {
    let film = &mut ctx.accounts.film;
    film.collection_mint = collection_mint;
    film.label = label;
    film.actor = actor;

    Ok(())
}

pub fn connect_film_to_label(ctx: Context<ConnectFilmToLabel>, label: Pubkey, film_pda: Pubkey) -> Result<()> {
    let label = &mut ctx.accounts.label;
    label.films.push(film_pda);

    Ok(())
}

pub fn push_history(ctx: Context<PushHistory>, collection_mint: Pubkey) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;

    if user_profile.history.contains(&collection_mint) {
        return Err(ProgramError::InvalidArgument.into());
    }

    user_profile.history.push(collection_mint);

    Ok(())
}