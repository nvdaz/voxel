use std::sync::Arc;

use bevy::utils::HashMap;
use futures_lite::future::{block_on, poll_once};
use futures_util::{
    future::{BoxFuture, Shared},
    FutureExt,
};
use ilattice::prelude::Extent;
use ndshape::ConstShape;
use noise::{NoiseFn, OpenSimplex};

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
    let simplex = OpenSimplex::new(0);
    let mut heightmap = Heightmap::new();

    for offset in Extent::from_min_and_shape(UVec2::ZERO, FlatChunkShape::ARRAY.into()).iter2() {
        let position = (origin * CHUNK_SIZE as i32) + offset.as_ivec2();

        let height = simplex
            .get((position.as_dvec2() / 50.0).to_array())
            .mul_add(40.0, 4.0);

        *heightmap.get_mut(offset) = height as i32;
    }

    Arc::new(heightmap)
}
