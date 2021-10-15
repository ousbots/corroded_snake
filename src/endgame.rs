use crate::events::*;
use crate::food::*;
use crate::materials::*;
use crate::snake::*;
use bevy::prelude::*;

/// Despawns everything in the game that shouldn't be included in a new game.
pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<Materials>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    heads: Query<Entity, With<SnakeHead>>,
) {
    if reader.iter().next().is_some() {
        for entity in food.iter().chain(segments.iter()).chain(heads.iter()) {
            commands.entity(entity).despawn();
        }

        spawn_snake(commands, materials);
    }
}
