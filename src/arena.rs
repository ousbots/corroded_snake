use bevy::prelude::*;
use rand::prelude::random;

// Arena size constants.
pub const ARENA_HEIGHT: u32 = 80;
pub const ARENA_WIDTH: u32 = 80;

/// An arena position in the game.
#[derive(Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn following(self: Self, heading: Direction) -> Position {
        let mut position = self.clone();

        match heading {
            Direction::Left => position.x += 1,
            Direction::Right => position.x -= 1,
            Direction::Up => position.y -= 1,
            Direction::Down => position.y += 1,
        }

        Position::warp_if_needed(&mut position);
        position
    }

    /// Returns a random position inside the arena.
    pub fn random() -> Position {
        Position {
            x: (random::<u32>() % ARENA_WIDTH) as i32,
            y: (random::<u32>() % ARENA_HEIGHT) as i32,
        }
    }

    pub fn warp_if_needed(mut position: &mut Position) {
        if position.x >= ARENA_WIDTH as i32 {
            position.x = 0;
        }

        if position.x < 0 {
            position.x = (ARENA_WIDTH - 1) as i32;
        }

        if position.y >= ARENA_HEIGHT as i32 {
            position.y = 0;
        }

        if position.y < 0 {
            position.y = (ARENA_HEIGHT - 1) as i32;
        }
    }
}

/// Specifies the area of an entity in grid units.
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    /// Returns a square Size of the given size.
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

/// Which direction an entity is moving.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Returns the opposite direction of the current one.
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    /// Returns a random direction.
    pub fn random() -> Self {
        match random::<i32>() % 4 {
            1 => Direction::Left,
            2 => Direction::Right,
            3 => Direction::Up,
            4 => Direction::Down,
            _ => Direction::Up,
        }
    }
}

/// Converts arena sizes to window sizes.
pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        )
    }
}

/// Converts arena positions to window positions.
pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
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
