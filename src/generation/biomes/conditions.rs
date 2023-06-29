use bevy::utils::HashMap;
use strum::IntoEnumIterator;

use super::Biome;

#[derive(Clone, Copy)]
pub struct BiomeConditions {
    pub temperature: f32,
    pub humidity: f32,
}

impl BiomeConditions {
    pub fn difference(self, other: Self) -> f32 {
        ((self.temperature - other.temperature).powi(2) + (self.humidity - other.humidity).powi(2))
            .sqrt()
    }

    pub fn get_biomes(self) -> HashMap<Biome, f32> {
        let mut absolute_map = HashMap::new();
        let mut total: f32 = 0.0;

        for biome in Biome::iter() {
            let difference = self.difference(biome.get_conditions());
            absolute_map.insert(biome, difference);
            total += difference;
        }

        absolute_map.retain(|_, difference| *difference / total < 0.01);

        let mut relative_map = HashMap::new();

        for (biome, difference) in absolute_map {
            relative_map.insert(biome, 1.0 - difference / total);
        }

        relative_map
    }
}
