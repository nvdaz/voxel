mod biomes;
pub mod chunk;
pub mod conditions;
pub mod heightmap;

use crate::prelude::*;

use self::{chunk::ChunkGenerationPlugin, heightmap::HeightmapGenerationPlugin};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GenerationSettings>()
            .add_plugin(ChunkGenerationPlugin)
            .add_plugin(HeightmapGenerationPlugin);
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
