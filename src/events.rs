use bevy::prelude::*;

/// Signals when an entity should grow.
pub struct GrowthEvent {
    pub entity: Entity,
}

/// Signals when game over conditions have been met.
pub struct GameOverEvent;
