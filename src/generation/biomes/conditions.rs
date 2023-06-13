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
}
