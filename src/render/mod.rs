pub mod lighting;
pub mod mesh;

use crate::prelude::*;

use self::{
    lighting::LightingPlugin,
    mesh::{chunk::MeshChunkPlugin, heightmap::MeshHeightmapPlugin},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderSettings>().add_plugins((
            MeshChunkPlugin,
            MeshHeightmapPlugin,
            LightingPlugin,
        ));
    }
}

#[derive(Resource)]
pub struct RenderSettings {
    /// The radius within which chunks are loaded.
    pub view_radius: UVec3,
    /// The buffer between the loading radius and drop radius.
    pub drop_padding: u32,
    /// The maximum number of concurrent meshing tasks.
    pub max_mesh_tasks: usize,
    /// The radius within which heightmaps are loaded.
    pub far_view_radius: UVec2,
}

#[cfg(debug_assertions)]
impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            view_radius: UVec3::new(8, 4, 8),
            drop_padding: 2,
            max_mesh_tasks: 32,
            far_view_radius: UVec2::splat(12),
        }
    }
}

#[cfg(not(debug_assertions))]
impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            view_radius: UVec3::new(16, 4, 16),
            drop_padding: 2,
            max_mesh_tasks: 32,
            far_view_radius: UVec2::splat(20),
        }
    }
}
