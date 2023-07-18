pub mod associated_ord;
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

use bevy::window::PresentMode;
use generation::GenerationPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;
use ui::UiPlugin;
use world::WorldPlugin;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Voxels".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            StoragePlugin,
            PlayerPlugin,
            GenerationPlugin,
            RenderPlugin,
            WorldPlugin,
            UiPlugin,
        ))
        .run();
}
