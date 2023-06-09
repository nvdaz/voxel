pub mod chunk;

use crate::prelude::*;

use self::chunk::WorldChunkPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldChunkPlugin);
    }
}
