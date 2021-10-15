use crate::events::*;
use crate::food::*;
use crate::materials::*;
use crate::snake::*;
use bevy::app::AppExit;
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

/// Signals the game is over if exit keypresses are found.
pub fn game_over_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut endgame_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        endgame_writer.send(AppExit)
    }
}
