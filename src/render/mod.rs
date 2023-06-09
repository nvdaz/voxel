pub mod mesh;

use crate::prelude::*;

use self::mesh::MeshPlugin;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MeshPlugin);
    }
}
