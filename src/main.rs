use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod arena;
mod endgame;
mod events;
mod food;
mod materials;
mod snake;

use arena::*;
use endgame::*;
use food::*;
use materials::*;
use snake::*;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(1.0, 0.6902, 0.5215).into()),
        segment_material: materials.add(Color::rgb(0.9765, 0.8353, 0.6549).into()),
        food_material: materials.add(Color::rgb(0.7882, 0.5882, 0.8).into()),
    });
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "snake!".to_string(),
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        //.insert_resource(ClearColor(Color::rgb(0.5647, 0.6667, 0.7961)))
        .insert_resource(ClearColor(Color::rgb(0.3706, 0.3784, 0.4686)))
        .add_event::<events::GrowthEvent>()
        .add_event::<events::GameOverEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(
            snake_movement_input
                .system()
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system(game_over_input.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.12))
                .with_system(snake_movement.system().label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .system()
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .system()
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                ),
        )
        .add_system(game_over.system().after(SnakeMovement::Movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
