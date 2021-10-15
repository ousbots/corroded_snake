use crate::arena::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use crate::materials::*;
use bevy::prelude::*;
use rand::prelude::random;

/// Food entity.
pub struct Food;

/// Spawns food in a random position in the arena.
pub fn food_spawner(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}
