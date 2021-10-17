use crate::arena::{Direction, Position, Size};
use crate::events::*;
use crate::food::*;
use crate::materials::*;
use bevy::prelude::*;

/// Snake head component.
pub struct SnakeHead {
    direction: Direction,
}

/// Snake segment component.
pub struct SnakeSegment;

/// Snake segments collection component.
pub struct SnakeSegments {
    segments: Vec<Entity>,
}

/// Represents the stage of snake movement for ordering systems.
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

/// Spawns a new snake with a single tail component in a fixed position and direction..
/// TODO: randomize the starting position and direcion.
pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    let head_position = Position::random();
    let head_direction = Direction::random();

    let tail = spawn_segment(
        &mut commands,
        &materials.segment_material,
        head_position.following(head_direction),
    );

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(0.0, 0.0)),
            ..Default::default()
        })
        .insert(SnakeHead {
            direction: head_direction,
        })
        .insert(SnakeSegments {
            segments: vec![tail],
        })
        .insert(head_position)
        .insert(Size::square(0.8))
        .id();
}

/// Spawns a snake segment with the given material at the given position.
pub fn spawn_segment(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

/// Sends growth events when a snake collides with food.
pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<(Entity, &Position), With<SnakeHead>>,
) {
    for (head_entity, head_position) in head_positions.iter() {
        for (food_entity, food_position) in food_positions.iter() {
            if food_position == head_position {
                commands.entity(food_entity).despawn();
                growth_writer.send(GrowthEvent {
                    entity: head_entity,
                });
            }
        }
    }
}

/// Reads snake growth events and adds a segment to the end of the tail.
pub fn snake_growth(
    mut commands: Commands,
    mut snakes: Query<(Entity, &mut SnakeSegments)>,
    mut positions: Query<(Entity, &Position), With<SnakeSegment>>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<Materials>,
) {
    for event in growth_reader.iter() {
        if let Ok((_, mut segments)) = snakes.get_mut(event.entity) {
            if let Some(tail_entity) = segments.segments.last() {
                if let Ok((_, tail_position)) = positions.get_mut(*tail_entity) {
                    segments.segments.push(spawn_segment(
                        &mut commands,
                        &materials.segment_material,
                        *tail_position,
                    ));
                }
            }
        }
    }
}

/// Moves the head and all tail segments of the snake forward by one unit.
pub fn snake_movement(
    mut snakes: Query<(Entity, &SnakeHead, &SnakeSegments)>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    for (snake_entity, head, segments) in snakes.iter_mut() {
        let mut segment_positions = segments
            .segments
            .iter()
            .filter_map(|e| {
                if let Some(pos) = positions.get_mut(*e).ok() {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect::<Vec<Position>>();

        let mut head_position = positions.get_mut(snake_entity).unwrap();

        if segment_positions.contains(&head_position) {
            game_over_writer.send(GameOverEvent);
        }

        // The head position should be first so that the other segments get moved towards it.
        segment_positions.insert(0, *head_position);

        match &head.direction {
            Direction::Left => head_position.x -= 1,
            Direction::Right => head_position.x += 1,
            Direction::Up => head_position.y += 1,
            Direction::Down => head_position.y -= 1,
        }

        // The head position needs to be warped to the opposite side if it is outside the arena.
        Position::warp_if_needed(&mut head_position);

        // Adjust all segment positions by moving them to where the previous one is.
        segment_positions
            .iter()
            .zip(segments.segments.iter())
            .for_each(|(pos, segment)| {
                if let Some(mut e) = positions.get_mut(*segment).ok() {
                    *e = *pos
                }
            });
    }
}

/// Changes the snakes head direction based on input. Does not allow the snake to turn in the
/// opposite direction than it's travelling.
pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in heads.iter_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::A) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::J) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::L) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::I) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::K) {
            Direction::Down
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir
        }
    }
}
