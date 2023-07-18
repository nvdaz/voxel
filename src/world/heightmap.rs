use bevy::{
    math::Vec3A,
    render::primitives::Aabb,
    utils::{HashMap, HashSet},
};

use crate::{prelude::*, render::mesh::heightmap::MeshHeightmapQueue};

pub struct WorldHeightmapPlugin;

impl Plugin for WorldHeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeightmapEntityMap>()
            .init_resource::<LoadHeightmapQueue>()
            .init_resource::<DropHeightmapQueue>()
            .add_systems(
                Update,
                (
                    handle_load_chunk_queue,
                    handle_drop_chunk_queue,
                    handle_blocking,
                ),
            );
    }
}

#[derive(Component)]
pub struct HeightmapMarker {
    pub position: IVec2,
    pub minimum: i32,
    pub maximum: i32,
    pub blocking: HashSet<IVec3>,
}

#[derive(Bundle)]
pub struct HeightmapBundle {
    heightmap: HeightmapMarker,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    aabb: Aabb,
}

impl HeightmapBundle {
    fn new(position: IVec2, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            heightmap: HeightmapMarker {
                position,
                minimum: i32::MIN,
                maximum: i32::MAX,
                blocking: HashSet::new(),
            },
            mesh,
            material,
            transform: Transform::from_translation(
                (position * CHUNK_SIZE as i32 - 1).extend_y(0).as_vec3(),
            ),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            aabb: Aabb {
                center: Vec3A::new(
                    PADDED_CHUNK_SIZE as f32,
                    f32::INFINITY,
                    PADDED_CHUNK_SIZE as f32,
                ),
                half_extents: Vec3A::new(
                    PADDED_CHUNK_SIZE as f32,
                    f32::INFINITY,
                    PADDED_CHUNK_SIZE as f32,
                ),
            },
        }
    }
}

#[derive(Default, Resource)]
pub struct HeightmapEntityMap {
    map: HashMap<IVec2, Entity>,
}

impl HeightmapEntityMap {
    pub fn insert(&mut self, position: IVec2, entity: Entity) {
        self.map.insert(position, entity);
    }

    pub fn get(&self, position: &IVec2) -> Option<Entity> {
        self.map.get(position).cloned()
    }

    pub fn contains(&self, position: &IVec2) -> bool {
        self.map.contains_key(position)
    }

    pub fn remove(&mut self, position: &IVec2) -> Option<Entity> {
        self.map.remove(position)
    }

    pub fn keys(&self) -> impl Iterator<Item = &IVec2> + '_ {
        self.map.keys()
    }
}

pub struct LoadHeightmap;
pub struct DropHeightmap;

pub type LoadHeightmapQueue = UnorderedQueue<IVec2, LoadHeightmap>;
pub type DropHeightmapQueue = UnorderedQueue<IVec2, DropHeightmap>;

fn handle_load_chunk_queue(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_map: ResMut<HeightmapEntityMap>,
    mut queue: ResMut<LoadHeightmapQueue>,
    mut chunk_mesh_queue: ResMut<MeshHeightmapQueue>,
) {
    for position in queue.drain(..) {
        if !entity_map.map.contains_key(&position) {
            let material = materials.add(StandardMaterial::from(Color::rgb(1.0, 1.0, 1.0)));
            let entity = commands
                .spawn(HeightmapBundle::new(position, Handle::default(), material))
                .id();

            entity_map.map.insert(position, entity);

            chunk_mesh_queue.push(position);
        }
    }
}

fn handle_drop_chunk_queue(
    mut commands: Commands,
    mut entity_map: ResMut<HeightmapEntityMap>,
    mut queue: ResMut<DropHeightmapQueue>,
    mut chunk_mesh_queue: ResMut<MeshHeightmapQueue>,
) {
    for position in queue.drain(..) {
        chunk_mesh_queue.remove(&position);

        if entity_map.map.contains_key(&position) {
            if let Some(entity) = entity_map.map.remove(&position) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn handle_blocking(
    mut heightmaps: Query<(&HeightmapMarker, &mut Visibility), Changed<HeightmapMarker>>,
) {
    for (heightmap, mut visibility) in &mut heightmaps {
        let mut should_block = false;

        for position in &heightmap.blocking {
            if ((position.y + 1) * CHUNK_SIZE as i32) > heightmap.minimum
                && (position.y * CHUNK_SIZE as i32) < heightmap.maximum
            {
                should_block = true;
                break;
            }
        }

        *visibility = if should_block {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    }
}
