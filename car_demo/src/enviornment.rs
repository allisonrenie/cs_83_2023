use std::f32::consts::PI;

use bevy::prelude::*;

/// Builds basic environment with lighting. 
pub fn build_environment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    // add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // add point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 3.0),
        point_light: PointLight {
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadow_depth_bias: 0.1,
            shadow_normal_bias: 0.9,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // add ground plane
    let ground_transform = assets.load("floor.glb#Scene0");
    let mut ground_entity = commands.spawn(SceneBundle {
        scene: ground_transform,
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::new(1.0, 1.0, 1.0))
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            )),
        ..default()
    });
}
