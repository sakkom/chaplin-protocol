use anchor_lang::prelude::*;
use crate::processor::*;
use crate::context::*;
use crate::state::{Actor};

pub mod processor;
pub mod context;
pub mod state;

declare_id!("DbNKdE3k31kCUTgNCKgiMD3CHn4MrWiuPZ2Ey4nHrPuF");

#[program]
pub mod chaplin_protocol {

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        processor::initialize_vault(ctx)
    }

    pub fn create_label(ctx: Context<CreateLabel>, squad_key: Pubkey, bubblegum_tree: Pubkey) -> Result<()> {
        processor::create_label(ctx, squad_key, bubblegum_tree)
    }

    pub fn create_user(ctx: Context<CreateUser>, name: String) -> Result<()> {
        processor::create_user(ctx, name)
    }

    pub fn connect_label_to_user(ctx: Context<ConnectLabelToUser>, label: Pubkey) -> Result<()> {
        processor::connect_label_to_user(ctx, label)
    }

    pub fn create_film(ctx: Context<CreateFilm>, collection_mint: Pubkey, label: Pubkey, actor: Actor) -> Result<()> {
        processor::create_film(ctx, collection_mint, label, actor)
    }

    pub fn connect_film_to_label(ctx: Context<ConnectFilmToLabel>, label: Pubkey, film_pda: Pubkey) -> Result<()> {
        processor::connect_film_to_label(ctx, label, film_pda)
    }

    pub fn push_history(ctx: Context<PushHistory>, collection_mint: Pubkey) -> Result<()> {
        processor::push_history(ctx, collection_mint)
    }
}

