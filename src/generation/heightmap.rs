use std::sync::Arc;

use bevy::utils::HashMap;
use futures_lite::future::{block_on, poll_once};
use futures_util::{
    future::{BoxFuture, Shared},
    FutureExt,
};
use ilattice::prelude::Extent;
use ndshape::ConstShape;
use noise::{Clamp, Curve, Fbm, MultiFractal, NoiseFn, OpenSimplex};

use crate::prelude::*;

pub struct HeightmapGenerationPlugin;

impl Plugin for HeightmapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeightmapGenerationCache>()
            .add_system(process_cache);
    }
}

#[derive(Resource)]
pub struct HeightmapGenerationCache {
    tasks: HashMap<IVec2, Shared<BoxFuture<'static, Arc<Heightmap>>>>,
    cache: HashMap<IVec2, Arc<Heightmap>>, // TODO: clear old values
}

impl Default for HeightmapGenerationCache {
    fn default() -> Self {
        Self {
            tasks: HashMap::with_capacity(64),
            cache: HashMap::with_capacity(128),
        }
    }
}

impl HeightmapGenerationCache {
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    pub fn tasks_len(&self) -> usize {
        self.tasks.len()
    }

    fn get_cache(&self, position: &IVec2) -> Option<&Arc<Heightmap>> {
        self.cache.get(position)
    }

    fn get_task(&self, position: &IVec2) -> Option<&Shared<BoxFuture<'static, Arc<Heightmap>>>> {
        self.tasks.get(position)
    }
}

fn process_cache(mut cache: ResMut<HeightmapGenerationCache>) {
    let mut completed_tasks = HashMap::new();
    for (position, task) in &mut cache.tasks {
        if let Some(heightmap) = block_on(poll_once(task)) {
            completed_tasks.insert(*position, heightmap);
        }
    }

    for position in completed_tasks.keys() {
        cache.tasks.remove(position);
    }
    cache.cache.extend(completed_tasks.into_iter());
}

pub enum GenerateHeightmapResult {
    CacheHit(Arc<Heightmap>),
    Loading(Shared<BoxFuture<'static, Arc<Heightmap>>>),
}

pub fn generate_heightmap(
    cache: &mut HeightmapGenerationCache,
    position: IVec2,
) -> GenerateHeightmapResult {
    if let Some(heightmap) = cache.cache.get(&position) {
        GenerateHeightmapResult::CacheHit(heightmap.clone())
    } else if let Some(future) = cache.tasks.get(&position) {
        GenerateHeightmapResult::Loading(future.clone())
    } else {
        let future = generate_heightmap_impl(position).boxed().shared();

        cache.tasks.insert(position, future.clone());

        GenerateHeightmapResult::Loading(future)
    }
}

async fn generate_heightmap_impl(origin: IVec2) -> Arc<Heightmap> {
    let simplex = Fbm::<OpenSimplex>::new(0)
        .set_octaves(4)
        .set_frequency(0.005)
        .set_persistence(0.5)
        .set_lacunarity(2.0);

    let noise = Curve::new(simplex)
        .add_control_point(-1.0, 0.0)
        .add_control_point(-0.8, 0.0)
        .add_control_point(-0.75, -0.25)
        .add_control_point(-0.7, 0.0)
        .add_control_point(0.25, 0.0)
        .add_control_point(0.5, 0.75)
        .add_control_point(1.0, 1.0);

    // let rivers_simplex = Fbm::<OpenSimplex>::new(1)
    //     .set_octaves(1)
    //     .set_frequency(0.005)
    //     .set_persistence(0.5)
    //     .set_lacunarity(2.0);

    // let rivers = Curve::new(rivers_simplex)
    //     .add_control_point(-1.0, -1.0)
    //     .add_control_point(-0.05, -1.0)
    //     .add_control_point(0.05, 0.0)
    //     .add_control_point(0.05, -1.0)
    //     .add_control_point(1.0, -1.0);

    let noise = Clamp::new(noise).set_bounds(-1.0, 1.0);
    // let rivers = Clamp::new(rivers).set_bounds(-1.0, 0.0);

    let mut heightmap = Heightmap::new();

    for offset in Extent::from_min_and_shape(UVec2::ZERO, FlatChunkShape::ARRAY.into()).iter2() {
        let position = (origin * CHUNK_SIZE as i32) + offset.as_ivec2();

        let dposition = position.as_dvec2().to_array();

        let height = noise.get(dposition).mul_add(100.0, 0.0);

        *heightmap.get_mut(offset) = height as i32;
    }

    Arc::new(heightmap)
}
