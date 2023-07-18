use std::{cmp, sync::Arc};

use bevy::{
    math::Vec3Swizzles,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future::{block_on, poll_once};

use crate::{
    generation::{chunk::ChunkGenerator, world::VoxelWorldGenerator},
    prelude::*,
    render::RenderSettings,
    world::{
        chunk::Chunk,
        heightmap::{HeightmapEntityMap, HeightmapMarker},
    },
};

pub struct MeshHeightmapPlugin;

impl Plugin for MeshHeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeshHeightmapQueue>().add_systems(
            Update,
            (handle_mesh_queue, handle_mesh_tasks, handle_chunk_load),
        );
    }
}

pub struct MeshHeightmap;

pub type MeshHeightmapQueue = DistanceOrderedQueue<IVec2, MeshHeightmap>;

#[derive(Component)]
pub struct MeshHeightmapTask {
    task: Task<GenerateHeightmapMeshResult>,
}

fn handle_mesh_queue(
    mut commands: Commands,
    mut queue: ResMut<MeshHeightmapQueue>,
    world_generator: Res<VoxelWorldGenerator>,
    entity_map: Res<HeightmapEntityMap>,
    tasks: Query<Entity, With<MeshHeightmapTask>>,
    settings: Res<RenderSettings>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    let mut i = 0;
    let to = cmp::min(
        settings.max_mesh_tasks.saturating_sub(tasks.iter().len()),
        queue.len(),
    );

    while let Some(position) = queue.pop() {
        if let Some(entity) = entity_map.get(&position) {
            let task = thread_pool.spawn(generate_heightmap_mesh_impl(
                world_generator.get(),
                position,
            ));

            commands.entity(entity).insert(MeshHeightmapTask { task });
        }

        i += 1;
        if i >= to {
            break;
        }
    }
}

fn handle_mesh_tasks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tasks: Query<(
        Entity,
        &mut HeightmapMarker,
        &mut Handle<Mesh>,
        &mut MeshHeightmapTask,
    )>,
) {
    for (entity, mut heightmap, mut handle, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.task)) {
            commands.entity(entity).remove::<MeshHeightmapTask>();
            heightmap.minimum = result.minimum;
            heightmap.maximum = result.maximum;
            if let Some(mesh_handle) = meshes.get_mut(&handle) {
                *mesh_handle = result.mesh;
            } else {
                *handle = meshes.add(result.mesh);
            }
        }
    }
}

fn handle_chunk_load(
    chunks: Query<&Chunk, Changed<Chunk>>,
    mut heightmaps: Query<&mut HeightmapMarker>,
    heightmap_entity_map: Res<HeightmapEntityMap>,
) {
    for chunk in &chunks {
        if chunk.is_loaded {
            if let Some(entity) = heightmap_entity_map.get(&chunk.position.xz()) {
                if let Ok(mut heightmap) = heightmaps.get_mut(entity) {
                    heightmap.blocking.insert(chunk.position);
                }
            }
        }
    }
}

struct GenerateHeightmapMeshResult {
    mesh: Mesh,
    minimum: i32,
    maximum: i32,
}

async fn generate_heightmap_mesh_impl(
    chunk_generator: Arc<ChunkGenerator>,
    position: IVec2,
) -> GenerateHeightmapMeshResult {
    let heightmap = chunk_generator.generate_heightmap(position).await;

    generate_heightmap_mesh(&heightmap).await
}

async fn generate_heightmap_mesh(heightmap: &Heightmap) -> GenerateHeightmapMeshResult {
    let subdivisions = 16;
    let size = CHUNK_SIZE as f32;
    let z_vertex_count = subdivisions + 2;
    let x_vertex_count = subdivisions + 2;
    let num_vertices = (z_vertex_count * x_vertex_count) as usize;
    let num_indices = ((z_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;
    let up = Vec3::Y.to_array();

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut colors = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

    let mut minimum = i32::MAX;
    let mut maximum = i32::MIN;

    for z in 0..z_vertex_count {
        for x in 0..x_vertex_count {
            let tx = x as f32 / (x_vertex_count - 1) as f32;
            let tz = z as f32 / (z_vertex_count - 1) as f32;
            let nearest_z = z / 4 * 4 * 4;
            let nearest_x = x / 4 * 4 * 4;
            let height = heightmap.get(UVec2::new(nearest_x, nearest_z));

            if minimum > height {
                minimum = height;
            }
            if maximum < height {
                maximum = height;
            }

            positions.push([tx * size, height as f32, tz * size]);
            colors.push(Voxel(3).get_color().as_rgba_f32());
            normals.push(up);
            uvs.push([tx, tz]);
        }
    }

    for y in 0..z_vertex_count - 1 {
        for x in 0..x_vertex_count - 1 {
            let quad = y * x_vertex_count + x;
            indices.push(quad + x_vertex_count + 1);
            indices.push(quad + 1);
            indices.push(quad + x_vertex_count);
            indices.push(quad);
            indices.push(quad + x_vertex_count);
            indices.push(quad + 1);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    GenerateHeightmapMeshResult {
        mesh,
        minimum,
        maximum,
    }
}
