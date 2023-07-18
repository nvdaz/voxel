pub mod chunk;
pub mod heightmap;

use crate::prelude::*;

use self::{chunk::WorldChunkPlugin, heightmap::WorldHeightmapPlugin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldChunkPlugin, WorldHeightmapPlugin));
    }
}
