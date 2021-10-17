use crate::arena::{Position, Size};
use crate::materials::*;
use crate::snake::*;
use bevy::prelude::*;

/// Food entity.
pub struct Food;

/// Spawns food in a random position in the arena.
pub fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    query: Query<&Position, With<(Food, SnakeSegment)>>,
) {
    let position = Position::random();
    let segments: Vec<&Position> = query.iter().map(|x| x).collect();

    if !segments.contains(&&position) {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .insert(Food)
            .insert(position)
            .insert(Size::square(0.8));
    }
}
