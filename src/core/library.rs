use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;

use crate::characters::ani::AniType;

#[derive(Deserialize, Clone, Debug)]
pub struct AnimationDetails {
    pub action_name: String,
    pub x: usize,
    pub y: usize,
    pub count: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ani {
    pub ani_type: AniType,
    pub texture_path: String,
    pub width: u32,
    pub height: u32,
    pub animations: Vec<AnimationDetails>,
}

pub fn build_library(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: &Ani,
    fps: u32,
) -> Vec<(AnimationId, Handle<TextureAtlasLayout>)> {
    // Create the spritesheet
    let column = ani.animations.iter().map(|e| e.count).max().unwrap_or(0);
    let spritesheet = Spritesheet::new(column, ani.animations.len());

    // Register animations
    let animations = &ani.animations;
    let sprite_width = ani.width;
    let sprite_height = ani.height;

    animations
        .iter()
        .map(|anim| {
            let clip = Clip::from_frames(spritesheet.horizontal_strip(anim.x, anim.y, anim.count))
                .with_duration(AnimationDuration::PerFrame(fps));
            let clip_id = library.register_clip(clip);
            let animation = Animation::from_clip(clip_id);
            // TODO use get_animation_name
            let animation_name = format!("{}_{}", ani.ani_type, &anim.action_name);

            // Check if the animation with the same name already exists
            if let Some(existing_animation_id) = library.animation_with_name(&animation_name) {
                (
                    existing_animation_id,
                    atlas_layouts.add(spritesheet.atlas_layout(sprite_width, sprite_height)),
                )
            } else {
                let animation_id = library.register_animation(animation);
                library
                    .name_animation(animation_id, animation_name)
                    .unwrap();
                (
                    animation_id,
                    atlas_layouts.add(spritesheet.atlas_layout(sprite_width, sprite_height)),
                )
            }
        })
        .collect::<Vec<_>>()
}
