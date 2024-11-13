use aoc_2022_18::{parse_voxel, Voxel, INPUT};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         shadows_enabled: true,
    //         intensity: 10_000_000.,
    //         range: 100.0,
    //         shadow_depth_bias: 0.2,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(40.0, 80.0, 40.0),
    //     ..default()
    // });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(30.0, 10.0, 30.0)
                .looking_at(Vec3::new(0.0, -0.5, 0.0), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // Voxel
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let hsla = Hsla::new(24.0, 1.0, 0.5, 0.5);
    let voxels = parse_voxel(INPUT);

    for Voxel { x, y, z } in voxels {
        commands.spawn(PbrBundle {
            mesh: cube.clone(),
            material: materials.add(Color::from(hsla)),
            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
            ..default()
        });
    }
}
