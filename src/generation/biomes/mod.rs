use std::{ops::Sub, sync::Arc};

use bevy::utils::HashMap;
use phf::phf_map;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy)]
pub struct BiomeConditions {
    temperature: f32,
    humidity: f32,
}

impl Sub for BiomeConditions {
    type Output = BiomeConditions;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            temperature: self.temperature - rhs.temperature,
            humidity: self.humidity - rhs.humidity,
        }
    }
}

pub trait BiomeGenerator {
    fn get_biome(&self) -> Biome;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Biome {
    TropicalRainforest,
    TemperateRainforest,
    Savannah,
    TemperateDeciduousForest,
    Taiga,
    Chaparral,
    SubtropicalDesert,
    ColdDesert,
    Tundra,
}

impl Biome {
    fn get_conditions(&self) -> BiomeConditions {
        match self {
            Biome::TropicalRainforest => BiomeConditions {
                temperature: 0.90,
                humidity: 0.75,
            },
            Biome::TemperateRainforest => BiomeConditions {
                temperature: 0.60,
                humidity: 0.60,
            },
            Biome::Savannah => BiomeConditions {
                temperature: 0.90,
                humidity: 0.40,
            },
            Biome::TemperateDeciduousForest => BiomeConditions {
                temperature: 0.60,
                humidity: 0.30,
            },
            Biome::Taiga => BiomeConditions {
                temperature: 0.20,
                humidity: 0.20,
            },
            Biome::Chaparral => BiomeConditions {
                temperature: 0.50,
                humidity: 0.10,
            },
            Biome::SubtropicalDesert => BiomeConditions {
                temperature: 0.90,
                humidity: 0.05,
            },
            Biome::ColdDesert => BiomeConditions {
                temperature: 0.50,
                humidity: 0.05,
            },
            Biome::Tundra => BiomeConditions {
                temperature: 0.0,
                humidity: 0.0,
            },
        }
    }
}

const BIOME_REGISTRY: phf::Map<&'static Biome, Arc<dyn BiomeGenerator>> = phf_map! {};

fn get_biomes(conditions: BiomeConditions) -> HashMap<Biome, f32> {
    let mut absolute_map = HashMap::new();
    let mut total: f32 = 0.0;

    for biome in Biome::iter() {
        let difference = biome.get_conditions() - conditions;
        let difference_squared =
            (difference.humidity.powi(2) + difference.temperature.powi(2)).sqrt();
        absolute_map.insert(biome, difference_squared);
        total += difference_squared
    }

    absolute_map.retain(|_, difference| *difference / total < 0.01);

    let mut relative_map = HashMap::new();

    for (biome, difference) in absolute_map {
        relative_map.insert(biome, 1.0 - difference / total);
    }

    relative_map
}
