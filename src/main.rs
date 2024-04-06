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

#[derive(Component)]
struct Player {
    speed: f32,
    is_flying: bool,
    is_on_ground: bool,
    jump_force: f32,
    gravity: f32,
    velocity: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            is_flying: false,
            is_on_ground: true,
            jump_force: 10.0,
            gravity: -9.81,
            velocity: Vec3::ZERO,
        }
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Player), With<Camera>>,
) {
    let mut delta_rotation = Vec2::ZERO;

    for event in mouse_motion_events.read() {
        delta_rotation += event.delta;
    }

    let mouse_sensitivity: f32 = 0.2;
    let yaw = delta_rotation.x * mouse_sensitivity.to_radians();
    let pitch = delta_rotation.y * mouse_sensitivity.to_radians();

    let delta_time = time.delta_seconds();

    for (mut transform, mut player) in query.iter_mut() {
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

        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_on_ground {
                player.velocity.y += player.jump_force;
                player.is_on_ground = false;
            } else if !player.is_flying {
                player.is_flying = true;
                player.velocity.y = 0.0;
            } else {
                player.is_flying = false;
            }
        }

        if !player.is_flying {
            player.velocity.y += player.gravity * delta_time;
        }

        transform.translation += player.velocity * delta_time;
        if transform.translation.y <= 0.0 {
            player.is_on_ground = true;
            player.velocity.y = 0.0;
            transform.translation.y = 0.0;
        } else {
            player.is_on_ground = false;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize() * player.speed;
        }

        let forward = transform.forward();
        let right = transform.right();
        transform.translation += (forward * direction.z + right * direction.x) * delta_time;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..std::default::Default::default()
    }).insert(Player::default());

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
    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "aqua waifu".to_string(),
            present_mode: bevy::window::PresentMode::Immediate,
            resizable: true,
            cursor: bevy::window::Cursor::default(),
            position: bevy::window::WindowPosition::Centered(bevy::window::MonitorSelection::Primary),
            resolution: bevy::window::WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
            name: None,
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    }

    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_movement);
    app.run();
}