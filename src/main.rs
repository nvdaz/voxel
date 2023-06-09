pub mod generation;
pub mod player;
mod prelude;
pub mod render;
pub mod storage;
mod trait_ext;
pub mod world;

use generation::GenerationPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;
use world::WorldPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StoragePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GenerationPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(WorldPlugin)
        .run();
}
