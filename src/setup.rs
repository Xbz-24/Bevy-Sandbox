use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::pbr::{PbrBundle, PointLightBundle, StandardMaterial};
use bevy::prelude::{BackgroundColor, Camera3dBundle, Color, Commands, default, GlobalTransform, ImageBundle, Mesh, Node, PositionType, Query, Res, ResMut, SceneBundle, shape, Style, Transform, UiImage, Val, Visibility, Window, With, ZIndex};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::ui::{ContentSize, FocusPolicy, widget};
use bevy::window::PrimaryWindow;
use crate::player::Player;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
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

    let window = window_query.single();
    let (window_width, window_height) = (window.width(), window.height());

    let crosshair_texture = asset_server.load("images/crosshair.png");
    let nyaaaa = Vec2::new(16.0, 16.0);
    commands.spawn(ImageBundle {
        node: Node::default(),
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(window_height / 2.0 - 8.0),
            left: Val::Px(window_width / 2.0 - 8.0),
            ..Default::default()
        },
        calculated_size: ContentSize::fixed_size(nyaaaa),
        background_color: BackgroundColor::DEFAULT,
        image: UiImage::new(crosshair_texture),
        image_size: widget::UiImageSize::default(),
        focus_policy: FocusPolicy::Pass,
        transform: Transform::default(),
        global_transform: GlobalTransform::default(),
        visibility: Visibility::Visible,
        z_index: ZIndex::default(),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("sponza/sponza.glb#Scene0"),
        ..default()
    });
}
