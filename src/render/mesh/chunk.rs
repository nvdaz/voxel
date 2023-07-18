use std::{
    cell::RefCell,
    cmp,
    sync::{Arc, RwLock},
};

use bevy::{
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use futures_lite::future::{block_on, poll_once};
use ndshape::ConstShape;
use once_cell::sync::Lazy;
use thread_local::ThreadLocal;

use crate::{
    player::PlayerCamera,
    prelude::*,
    world::chunk::{Chunk, ChunkEntityMap},
};

use crate::render::RenderSettings;

pub struct MeshChunkPlugin;

impl Plugin for MeshChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeshChunkQueue>().add_systems(
            Update,
            (handle_mesh_queue, handle_mesh_tasks, update_center),
        );
    }
}

pub struct MeshChunk;

pub type MeshChunkQueue = DistanceOrderedQueue<IVec3, MeshChunk>;

#[derive(Component)]
pub struct MeshChunkTask {
    task: Task<Mesh>,
}

static SHARED_GREEDY_BUFFER: Lazy<ThreadLocal<RefCell<GreedyQuadsBuffer>>> =
    Lazy::new(ThreadLocal::default);

fn handle_mesh_queue(
    mut commands: Commands,
    mut queue: ResMut<MeshChunkQueue>,
    entity_map: Res<ChunkEntityMap>,
    world: Res<VoxelWorld>,
    tasks: Query<Entity, With<MeshChunkTask>>,
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
            if let Some(chunk) = world.get(&position) {
                let task = thread_pool.spawn(generate_chunk_mesh_impl(chunk));

                commands.entity(entity).insert(MeshChunkTask { task });
            }
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
    mut tasks: Query<(Entity, &mut Chunk, &mut Handle<Mesh>, &mut MeshChunkTask), With<Chunk>>,
) {
    for (entity, mut chunk, mut handle, mut task) in &mut tasks {
        if let Some(mesh) = block_on(poll_once(&mut task.task)) {
            commands.entity(entity).remove::<MeshChunkTask>();
            if let Some(mesh_handle) = meshes.get_mut(&handle) {
                *mesh_handle = mesh;
            } else {
                *handle = meshes.add(mesh);
            }
            chunk.is_loaded = true;
        }
    }
}

fn update_center(
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    mut queue: ResMut<MeshChunkQueue>,
) {
    let camera = camera.single();

    let center = camera.translation().as_ivec3() / CHUNK_SIZE as i32;

    queue.update_center(center)
}

async fn generate_chunk_mesh_impl(chunk: Arc<RwLock<VoxelChunk>>) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut greedy_buffer = SHARED_GREEDY_BUFFER
        .get_or(|| RefCell::new(GreedyQuadsBuffer::new(ChunkShape::USIZE)))
        .borrow_mut();
    greedy_buffer.reset(ChunkShape::USIZE);

    let voxels = &chunk.read().unwrap().voxels;

    greedy_quads(
        voxels.read_data(),
        &ChunkShape {},
        [0; 3],
        [PADDED_CHUNK_SIZE - 1; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut greedy_buffer,
    );

    let num_quads = greedy_buffer.quads.num_quads();
    let num_indices = num_quads * 6;
    let num_vertices = num_quads * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut colors = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);

    for (group, face) in greedy_buffer
        .quads
        .groups
        .iter()
        .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter())
    {
        for quad in group.iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            colors.extend_from_slice(
                &[voxels
                    .voxel_at(UVec3::from_array(quad.minimum))
                    .get_color()
                    .as_rgba_f32(); 4],
            );
        }
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(colors),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
    );
    mesh.set_indices(Some(Indices::U32(indices.clone())));

    mesh
}
