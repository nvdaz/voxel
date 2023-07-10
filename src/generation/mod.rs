mod biomes;
pub mod chunk;
pub mod conditions;
pub mod terrain;

use crate::prelude::*;

use self::chunk::ChunkGenerationPlugin;

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GenerationSettings>()
            .add_plugin(ChunkGenerationPlugin);
    }
}

#[derive(Resource)]
pub struct GenerationSettings {
    max_generation_tasks: usize,
}

#[cfg(debug_assertions)]
impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            max_generation_tasks: 8,
        }
    }
}

#[cfg(not(debug_assertions))]
impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            max_generation_tasks: 128,
        }
    }
}
