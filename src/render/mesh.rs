use std::{
    cell::RefCell,
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
    prelude::*,
    world::chunk::{Chunk, ChunkEntityMap},
};

pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeshChunkQueue>()
            .add_systems((handle_mesh_queue, handle_mesh_tasks));
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct MeshChunk(IVec3);

impl From<IVec3> for MeshChunk {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

pub type MeshChunkQueue = UnorderedQueue<MeshChunk>;

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
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for MeshChunk(position) in queue.drain() {
        let entity = *entity_map.map.get(&position).unwrap();
        let chunk = world.get(&position).unwrap();
        let task = thread_pool.spawn(generate_chunk_mesh_impl(chunk));

        commands.entity(entity).insert(MeshChunkTask { task });
    }
}

fn handle_mesh_tasks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tasks: Query<(Entity, &mut Handle<Mesh>, &mut MeshChunkTask), With<Chunk>>,
) {
    for (entity, mut handle, mut task) in &mut tasks {
        if let Some(mesh) = block_on(poll_once(&mut task.task)) {
            commands.entity(entity).remove::<MeshChunkTask>();
            if let Some(mesh_handle) = meshes.get_mut(&handle) {
                *mesh_handle = mesh;
            } else {
                *handle = meshes.add(mesh);
            }
        }
    }
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
