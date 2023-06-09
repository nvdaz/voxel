use bevy::{
    math::Vec3A,
    render::primitives::Aabb,
    utils::{HashMap, HashSet},
};

use crate::{generation::chunk::ChunkGenerationQueue, prelude::*};

pub struct WorldChunkPlugin;

impl Plugin for WorldChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkEntityMap>()
            .init_resource::<LoadChunkQueue>()
            .add_system(handle_load_chunk_queue);
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

#[derive(Default, Resource)]
pub struct LoadChunkQueue {
    pub queue: HashSet<IVec3>,
}

fn handle_load_chunk_queue(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_map: ResMut<ChunkEntityMap>,
    mut queue: ResMut<LoadChunkQueue>,
    world: Res<VoxelWorld>,
    mut chunk_gen_queue: ResMut<ChunkGenerationQueue>,
) {
    for position in queue.queue.drain() {
        if !entity_map.map.contains_key(&position) {
            if !world.contains(&position) {
                chunk_gen_queue.queue.insert(position);
            } // TODO: else -> mark as dirty
            let material = materials.add(StandardMaterial::from(Color::rgb(0.5, 0.0, 0.5)));
            let entity = commands
                .spawn(ChunkBundle::new(position, Handle::default(), material))
                .id();

            entity_map.map.insert(position, entity);
        }
    }
}
