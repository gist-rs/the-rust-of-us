use anyhow::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_yaml::from_str;
use std::fs;

use crate::timeline::init::LookDirection;

use super::{play::Act, setup::CharacterId};

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Deserialize, Default, Debug)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub npcs: Vec<Npc>,
}

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Deserialize, Debug)]
pub struct Player {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub look_direction: LookDirection,
    pub act: Act,
    pub attack: u32,
    pub defend: u32,
    pub health: u32,
    pub tasks: Vec<String>,
    pub mindsets: Vec<String>,
}

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Deserialize, Clone, Debug)]
pub struct Enemy {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub look_direction: LookDirection,
    pub act: Act,
    pub attack: u32,
    pub defend: u32,
    pub health: u32,
    pub mindsets: Vec<String>,
}

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Deserialize, Debug)]
pub struct Npc {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub look_direction: LookDirection,
    pub act: Act,
    pub prompt: String,
}

#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Default, Debug)]
pub struct GameStage(pub Stage);

pub fn load_stage_from_yaml(file_path: &str) -> Result<Stage> {
    let file_content = fs::read_to_string(file_path).expect("Expected stage_1-1.yml");
    let stage: Stage = from_str(&file_content)?;
    Ok(stage)
}

pub fn init_stage(mut commands: Commands) {
    let stage = load_stage_from_yaml("assets/stage_1-1.yml").expect("stage_1-1.yml");
    commands.insert_resource(GameStage(stage));
}
