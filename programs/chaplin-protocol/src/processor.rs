use crate::{context::*, state::Actor};
use anchor_lang::prelude::*;

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

    Ok(())
}

pub fn connect_label_to_user(ctx: Context<ConnectLabelToUser>, label: Pubkey) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.label.push(label.key());

    Ok(())
}

pub fn create_film(ctx: Context<CreateFilm>, collection_mint: Pubkey, label: Pubkey, actor: Actor) -> Result<()> {
    msg!("Creating film with collection_mint: {:?}", collection_mint);
    msg!("Label: {:?}", label);
    msg!("actor: {:?}", actor);

    let film = &mut ctx.accounts.film;
    film.collection_mint = collection_mint;
    film.label = label;
    film.actor = actor;

    Ok(())
}
