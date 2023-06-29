pub mod cache;
pub mod generation;
pub mod player;
mod prelude;
pub mod queue;
pub mod render;
pub mod storage;
mod trait_ext;
mod ui;
pub mod world;
pub mod associated_ord;

use generation::GenerationPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;
use ui::UiPlugin;
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
        .add_plugin(UiPlugin)
        .run();
}
