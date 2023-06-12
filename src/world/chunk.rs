use bevy::{math::Vec3A, render::primitives::Aabb, utils::HashMap};

use crate::{generation::chunk::ChunkGenerationQueue, prelude::*};

pub struct WorldChunkPlugin;

impl Plugin for WorldChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkEntityMap>()
            .init_resource::<LoadChunkQueue>()
            .init_resource::<DropChunkQueue>()
            .add_systems((handle_load_chunk_queue, handle_drop_chunk_queue));
    }
}

#[derive(Component)]
pub struct Chunk {
    pub position: IVec3,
}

#[derive(Bundle)]
pub struct ChunkBundle {
    chunk: Chunk,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    aabb: Aabb,
}

impl ChunkBundle {
    fn new(position: IVec3, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            chunk: Chunk { position },
            mesh,
            material,
            transform: Transform::from_translation((position * CHUNK_SIZE as i32 - 1).as_vec3()),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            aabb: Aabb {
                center: Vec3A::splat(PADDED_CHUNK_SIZE as f32),
                half_extents: Vec3A::splat(PADDED_CHUNK_SIZE as f32),
            },
        }
    }
}

#[derive(Default, Resource)]
pub struct ChunkEntityMap {
    pub map: HashMap<IVec3, Entity>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct LoadChunk(IVec3);

impl From<IVec3> for LoadChunk {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct DropChunk(IVec3);

impl From<IVec3> for DropChunk {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

pub type LoadChunkQueue = UnorderedQueue<LoadChunk>;
pub type DropChunkQueue = UnorderedQueue<DropChunk>;

fn handle_load_chunk_queue(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_map: ResMut<ChunkEntityMap>,
    mut queue: ResMut<LoadChunkQueue>,
    world: Res<VoxelWorld>,
    mut chunk_gen_queue: ResMut<ChunkGenerationQueue>,
) {
    for LoadChunk(position) in queue.drain() {
        if !entity_map.map.contains_key(&position) {
            if !world.contains(&position) {
                chunk_gen_queue.push(position);
            } // TODO: else -> mark as dirty
            let material = materials.add(StandardMaterial::from(Color::rgb(1.0, 1.0, 1.0)));
            let entity = commands
                .spawn(ChunkBundle::new(position, Handle::default(), material))
                .id();

            entity_map.map.insert(position, entity);
        }
    }
}

fn handle_drop_chunk_queue(
    mut commands: Commands,
    mut entity_map: ResMut<ChunkEntityMap>,
    mut queue: ResMut<DropChunkQueue>,
    mut world: ResMut<VoxelWorld>,
) {
    for DropChunk(position) in queue.drain() {
        if entity_map.map.contains_key(&position) {
            if let Some(chunk) = world.remove(&position) {
                // TODO: save
            }

            if let Some(entity) = entity_map.map.remove(&position) {
                commands.entity(entity).despawn();
            }
        }
    }
}
