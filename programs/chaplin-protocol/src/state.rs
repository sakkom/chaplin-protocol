use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use anchor_lang::AnchorDeserialize;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub label: Vec<Pubkey>,
    pub history: Vec<Pubkey>,
}

#[account]
pub struct Label {
    pub squad_key: Pubkey,
    pub bubblegum_tree: Pubkey, 
    pub films: Vec<Pubkey>
}

#[account]
pub struct Film {
    pub collection_mint: Pubkey,
    pub label: Pubkey,
    pub actor: Actor,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Actor {
    pub creator: Vec<Pubkey>,
    pub co_creator: Vec<Pubkey>
}



