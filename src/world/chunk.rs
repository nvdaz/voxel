use bevy::{
    math::{Vec3A, Vec3Swizzles},
    render::primitives::Aabb,
    utils::HashMap,
};

use crate::{
    generation::chunk::ChunkGenerationQueue, prelude::*, render::mesh::chunk::MeshChunkQueue,
};

use super::heightmap::{HeightmapEntityMap, HeightmapMarker};

pub struct WorldChunkPlugin;

impl Plugin for WorldChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkEntityMap>()
            .init_resource::<LoadChunkQueue>()
            .init_resource::<DropChunkQueue>()
            .add_systems(Update, (handle_load_chunk_queue, handle_drop_chunk_queue));
    }
}

#[derive(Component)]
pub struct Chunk {
    pub position: IVec3,
    pub is_loaded: bool,
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
            chunk: Chunk {
                position,
                is_loaded: false,
            },
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
    map: HashMap<IVec3, Entity>,
}

impl ChunkEntityMap {
    pub fn insert(&mut self, position: IVec3, entity: Entity) {
        self.map.insert(position, entity);
    }

    pub fn get(&self, position: &IVec3) -> Option<Entity> {
        self.map.get(position).cloned()
    }

    pub fn contains(&self, position: &IVec3) -> bool {
        self.map.contains_key(position)
    }

    pub fn remove(&mut self, position: &IVec3) -> Option<Entity> {
        self.map.remove(position)
    }

    pub fn keys(&self) -> impl Iterator<Item = &IVec3> + '_ {
        self.map.keys()
    }
}

pub struct LoadChunk;
pub struct DropChunk;

pub type LoadChunkQueue = UnorderedQueue<IVec3, LoadChunk>;
pub type DropChunkQueue = UnorderedQueue<IVec3, DropChunk>;

fn handle_load_chunk_queue(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_map: ResMut<ChunkEntityMap>,
    mut queue: ResMut<LoadChunkQueue>,
    world: Res<VoxelWorld>,
    mut chunk_gen_queue: ResMut<ChunkGenerationQueue>,
    mut chunk_mesh_queue: ResMut<MeshChunkQueue>,
) {
    for position in queue.drain(..) {
        if !entity_map.map.contains_key(&position) {
            let material = materials.add(StandardMaterial::from(Color::rgb(1.0, 1.0, 1.0)));
            let entity = commands
                .spawn(ChunkBundle::new(position, Handle::default(), material))
                .id();

            entity_map.map.insert(position, entity);

            if !world.contains(&position) {
                chunk_gen_queue.push(position);
            } else {
                chunk_mesh_queue.push(position);
            }
        }
    }
}

fn handle_drop_chunk_queue(
    mut commands: Commands,
    mut entity_map: ResMut<ChunkEntityMap>,
    mut queue: ResMut<DropChunkQueue>,
    mut world: ResMut<VoxelWorld>,
    mut chunk_gen_queue: ResMut<ChunkGenerationQueue>,
    mut chunk_mesh_queue: ResMut<MeshChunkQueue>,
    heightmap_entity_map: Res<HeightmapEntityMap>,
    mut heightmaps: Query<&mut HeightmapMarker>,
) {
    for position in queue.drain(..) {
        chunk_gen_queue.remove(&position);
        chunk_mesh_queue.remove(&position);

        if let Some(entity) = entity_map.map.remove(&position) {
            if let Some(chunk) = world.remove(&position) {
                // TODO: save
            }

            commands.entity(entity).despawn();
        }
        if let Some(entity) = heightmap_entity_map.get(&position.xz()) {
            if let Ok(mut heightmap) = heightmaps.get_mut(entity) {
                heightmap.blocking.remove(&position);
            }
        }
    }
}
