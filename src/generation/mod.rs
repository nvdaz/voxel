pub mod chunk;
pub mod heightmap;

use crate::prelude::*;

use self::{chunk::ChunkGenerationPlugin, heightmap::HeightmapGenerationPlugin};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ChunkGenerationPlugin)
            .add_plugin(HeightmapGenerationPlugin);
    }
}
