mod conditions;

use std::{ops::Sub, sync::Arc};

use bevy::utils::HashMap;
use phf::phf_map;
use strum::{EnumIter, IntoEnumIterator};

use self::conditions::BiomeConditions;

pub trait BiomeGenerator {
    fn get_biome(&self) -> Biome;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Biome {
    TropicalRainforest,
    // TemperateRainforest,
    // Savannah,
    // TemperateDeciduousForest,
    // Taiga,
    // Chaparral,
    // SubtropicalDesert,
    // ColdDesert,
    Tundra,
}

impl Biome {
    fn get_conditions(&self) -> BiomeConditions {
        match self {
            Biome::TropicalRainforest => BiomeConditions {
                temperature: 0.90,
                humidity: 0.75,
            },
            // Biome::TemperateRainforest => BiomeConditions {
            //     temperature: 0.60,
            //     humidity: 0.60,
            // },
            // Biome::Savannah => BiomeConditions {
            //     temperature: 0.90,
            //     humidity: 0.40,
            // },
            // Biome::TemperateDeciduousForest => BiomeConditions {
            //     temperature: 0.60,
            //     humidity: 0.30,
            // },
            // Biome::Taiga => BiomeConditions {
            //     temperature: 0.20,
            //     humidity: 0.20,
            // },
            // Biome::Chaparral => BiomeConditions {
            //     temperature: 0.50,
            //     humidity: 0.10,
            // },
            // Biome::SubtropicalDesert => BiomeConditions {
            //     temperature: 0.90,
            //     humidity: 0.05,
            // },
            // Biome::ColdDesert => BiomeConditions {
            //     temperature: 0.50,
            //     humidity: 0.05,
            // },
            Biome::Tundra => BiomeConditions {
                temperature: 0.0,
                humidity: 0.0,
            },
        }
    }

    fn generator(&self) -> &'static Box<dyn BiomeGenerator> {
        match self {
            _ => todo!(),
        }
    }
}

fn get_biomes(conditions: BiomeConditions) -> HashMap<Biome, f32> {
    let mut absolute_map = HashMap::new();
    let mut total: f32 = 0.0;

    for biome in Biome::iter() {
        let difference = BiomeConditions::difference(conditions, biome.get_conditions());
        absolute_map.insert(biome, difference);
        total += difference
    }

    absolute_map.retain(|_, difference| *difference / total < 0.01);

    let mut relative_map = HashMap::new();

    for (biome, difference) in absolute_map {
        relative_map.insert(biome, 1.0 - difference / total);
    }

    relative_map
}
