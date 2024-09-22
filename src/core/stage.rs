use anyhow::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_yaml::from_str;
use std::fs;

use super::setup::CharacterId;

#[allow(unused)]
#[derive(Deserialize, Default, Debug, Resource)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub npcs: Vec<Npc>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Player {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub attack: u32,
    pub defend: u32,
    pub health: u32,
    pub tasks: Vec<String>,
    pub mindsets: Vec<String>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Enemy {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub attack: u32,
    pub defend: u32,
    pub health: u32,
    pub mindsets: Vec<String>,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct Npc {
    pub r#type: String,
    pub character_id: CharacterId,
    pub position: String,
    pub prompt: String,
}

#[derive(Resource, Default, Debug)]
pub struct GameStage(pub Stage);

pub fn load_stage_from_yaml(file_path: &str) -> Result<Stage> {
    let file_content = fs::read_to_string(file_path).expect("Expected stage_1-1.yml");
    let stage: Stage = from_str(&file_content)?;
    Ok(stage)
}

pub fn init_stage(mut commands: Commands) {
    println!("🔥 init_stage");
    let stage = load_stage_from_yaml("assets/stage_1-1.yml").expect("stage_1-1.yml");
    commands.insert_resource(GameStage(stage));
}
