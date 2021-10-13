use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

// Arena size constants
const ARENA_HEIGHT: u32 = 50;
const ARENA_WIDTH: u32 = 50;

// Food "tag"
struct Food;

// Snake
struct SnakeHead {
    direction: Direction,
}

struct SnakeSegment;

struct SnakeSegments {
    segments: Vec<Entity>,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

// Game materials
struct Materials {
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>,
    food_material: Handle<ColorMaterial>,
}

// Position
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

// Direction
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// Events
struct GrowthEvent {
    entity: Entity,
}
struct GameOverEvent;

fn food_spawner(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.food_material.clone(),
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    let tail = spawn_segment(
        &mut commands,
        &materials.segment_material,
        Position { x: 3, y: 2 },
    );

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead {
            direction: Direction::Up,
        })
        .insert(SnakeSegments {
            segments: vec![tail],
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8))
        .id();
}

fn spawn_segment(
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

fn snake_movement(
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
        segment_positions.insert(0, *head_position);

        match &head.direction {
            Direction::Left => head_position.x -= 1,
            Direction::Right => head_position.x += 1,
            Direction::Up => head_position.y += 1,
            Direction::Down => head_position.y -= 1,
        }

        if segment_positions.contains(&head_position) {
            game_over_writer.send(GameOverEvent);
        }

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

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in heads.iter_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir
        }
    }
}

fn snake_eating(
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

fn snake_growth(
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

fn game_over(
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

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "snake!".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(
            snake_movement_input
                .system()
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
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
