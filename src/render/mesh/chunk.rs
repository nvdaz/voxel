use std::{
    cell::RefCell,
    cmp,
    sync::{Arc, RwLock},
};

use bevy::{
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh_pop::{
    greedy_quads, visible_faces_quads, LodMaterial, PopBuffer, QuadBuffer, VisitedBuffer,
};
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
    task: Task<([u32; 8], Mesh)>,
}

static SHARED_GREEDY_BUFFER: Lazy<ThreadLocal<RefCell<VisitedBuffer>>> =
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
    mut materials: ResMut<Assets<LodMaterial<6>>>,
    mut tasks: Query<
        (
            Entity,
            &mut Chunk,
            &mut Handle<Mesh>,
            &mut Handle<LodMaterial<6>>,
            &mut MeshChunkTask,
        ),
        With<Chunk>,
    >,
) {
    for (entity, mut chunk, mut handle, material, mut task) in &mut tasks {
        if let Some((buckets, mesh)) = block_on(poll_once(&mut task.task)) {
            commands.entity(entity).remove::<MeshChunkTask>();

            if let Some(material_handle) = materials.get_mut(&material) {
                material_handle.buckets = unsafe { std::mem::transmute(buckets) };
            }
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

async fn generate_chunk_mesh_impl(chunk: Arc<RwLock<VoxelChunk>>) -> ([u32; 8], Mesh) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut visited_buffer = SHARED_GREEDY_BUFFER
        .get_or(|| RefCell::new(VisitedBuffer::new(ChunkShape::USIZE)))
        .borrow_mut();

    let voxels = &chunk.read().unwrap().voxels;

    let mut buffer = PopBuffer::<6, _>::new();

    visible_faces_quads::<66, 66, 66, 6, _>(voxels.read_data(), &mut visited_buffer, &mut buffer);
    // greedy_quads::<66, 66, 66, 6, _>(voxels.read_data(), &mut visited_buffer, &mut buffer);

    let buckets = buffer.get_buckets();

    let num_quads = buffer.num_quads();
    let num_indices = num_quads * 6;
    let num_vertices = num_quads * 4;

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut colors = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);

    for (face, quad) in buffer.iter_quads() {
        indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
        positions.extend_from_slice(&face.quad_mesh_positions(quad, 0, 1.0));
        normals.extend_from_slice(&face.quad_mesh_normals());
        colors.extend_from_slice(&[voxels.voxel_at(quad.minimum).get_color().as_rgba_f32(); 4]);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0; 2]; num_vertices]);
    mesh.set_indices(Some(Indices::U32(indices.clone())));

    (buckets, mesh)
}
