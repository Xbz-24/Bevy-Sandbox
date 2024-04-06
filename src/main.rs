use std::default::Default;
use bevy::app::App;
use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::Commands;
use bevy::ecs::system::ResMut;
use bevy::input::mouse::MouseMotion;
use bevy::math::Vec3;
use bevy::pbr::PbrBundle;
use bevy::pbr::PointLightBundle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh::{Indices, PrimitiveTopology, shape};
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;
use bevy::utils::petgraph::visit::Walker;

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut delta_rotation = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta_rotation += event.delta;
    }
    let mouse_sensitivity: f32 = 0.2;
    let yaw = delta_rotation.x * mouse_sensitivity.to_radians();
    let pitch = delta_rotation.y * mouse_sensitivity.to_radians();

    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            direction.y -= 1.0;
        }
        transform.rotation *= Quat::from_rotation_y(yaw);
        let current_pitch = Quat::from_rotation_x(pitch);
        transform.rotation = current_pitch * transform.rotation;
        let speed = 5.0;
        let rotated_direction = transform.rotation.mul_vec3(direction);
        transform.translation += time.delta_seconds() * rotated_direction * speed;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Handle<Mesh>), With<Handle<Scene>>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..std::default::Default::default()
    });

    commands.spawn(SceneBundle{
        scene: asset_server.load("gltf/scene.gltf#Scene0"),
        ..default()
    });

    let vertices = [
        ([0.5, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]),
        ([0.5, -0.5, 0.5], [0.0, 1.0, 0.0, 1.0]),
        ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]),
        ([-0.5, 0.5, 0.5], [1.0, 1.0, 0.0, 1.0]),
        ([0.5, 0.5, -0.5], [1.0, 0.0, 1.0, 1.0]),
        ([0.5, -0.5, -0.5], [0.0, 1.0, 1.0, 1.0]),
        ([-0.5, -0.5, -0.5], [0.5, 0.5, 0.5, 1.0]),
        ([-0.5, 0.5, -0.5], [0.0, 0.0, 0.0, 1.0]),
    ];
    let vertex_positions: Vec<[f32; 3]> = vertices.iter()
        .map(|&(pos, _)| pos)
        .collect();

    let vertex_colors: Vec<[f32; 4]> = vertices.iter().map(|&(_, col)| col).collect();

    let indices = vec![
        0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 4, 7, 3, 4, 3, 0, 1, 2, 6, 1, 6, 5, 7, 6, 2, 7, 2, 3,
        0, 1, 5, 0, 5, 4,
    ];

    let colors = [
        Color::rgb(1.0, 0.0, 0.0),
        Color::rgb(0.0, 1.0, 0.0),
        Color::rgb(0.0, 0.0, 1.0),
    ];

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })
        ),
        material: materials.add(StandardMaterial{
            base_color: Color::rgb(0.5, 0.5, 1.0),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(1.5, 0.0, 0.0)),
        ..Default::default()
    });

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions, );
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors, );
    mesh.insert_indices(Indices::U32(indices), );

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }
    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_movement);
    app.run();
}