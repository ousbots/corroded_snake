use bevy::prelude::*;

/// Stores game materials.
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub segment_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}
