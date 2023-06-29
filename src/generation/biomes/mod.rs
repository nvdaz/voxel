mod conditions;

use strum::EnumIter;

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

    fn generator(&self) -> &'static dyn BiomeGenerator {
        match self {
            _ => todo!(),
        }
    }
}
