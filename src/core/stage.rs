use anyhow::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_yaml::from_str;
use std::{fs, slice::Iter};

use crate::characters::actions::{Act, LookDirection};

use super::setup::CharacterId;

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Resource))]
#[derive(Deserialize, Default, Debug)]
pub struct Stage {
    pub id: String,
    pub name: String,
    pub humans: Vec<Human>,
    pub enemies: Vec<Monster>,
    pub npcs: Vec<Npc>,
}

pub trait StageInfo {
    fn get_characters_iter_by_type<T>(&self) -> Option<Iter<T>>
    where
        T: 'static;
}

impl StageInfo for Stage {
    fn get_characters_iter_by_type<T>(&self) -> Option<Iter<T>>
    where
        T: 'static,
    {
        match std::any::TypeId::of::<T>() {
            id if id == std::any::TypeId::of::<Human>() => {
                let humans: &[T] = unsafe { std::mem::transmute(&self.humans[..]) };
                Some(humans.iter())
            }
            id if id == std::any::TypeId::of::<Monster>() => {
                let enemies: &[T] = unsafe { std::mem::transmute(&self.enemies[..]) };
                Some(enemies.iter())
            }
            id if id == std::any::TypeId::of::<Npc>() => {
                let npcs: &[T] = unsafe { std::mem::transmute(&self.npcs[..]) };
                Some(npcs.iter())
            }
            _ => None,
        }
    }
}

pub trait CharacterInfo: Component {
    fn r#type(&self) -> &String;
    fn character_id(&self) -> &CharacterId;
    fn position(&self) -> &String;
    fn look_direction(&self) -> &LookDirection;
    fn act(&self) -> &Act;
    fn get_clone(&self) -> Self;
}

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Deserialize, Clone, Debug)]
pub struct Human {
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

impl CharacterInfo for Human {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn character_id(&self) -> &CharacterId {
        &self.character_id
    }
    fn position(&self) -> &String {
        &self.position
    }
    fn look_direction(&self) -> &LookDirection {
        &self.look_direction
    }
    fn act(&self) -> &Act {
        &self.act
    }

    fn get_clone(&self) -> Self {
        self.clone()
    }
}

#[allow(unused)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
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

impl CharacterInfo for Monster {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn character_id(&self) -> &CharacterId {
        &self.character_id
    }
    fn position(&self) -> &String {
        &self.position
    }
    fn look_direction(&self) -> &LookDirection {
        &self.look_direction
    }
    fn act(&self) -> &Act {
        &self.act
    }

    fn get_clone(&self) -> Self {
        self.clone()
    }
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
