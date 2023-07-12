use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin, pbr::DirectionalLightShadowMap,
};

use crate::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TemporalAntiAliasPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 100000.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.25,
    });
}
