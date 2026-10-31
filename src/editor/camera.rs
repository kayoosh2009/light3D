use bevy::prelude::*;

#[derive(Component)]
pub struct EditorCamera;

pub fn spawn_editor_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D-камера
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        EditorCamera,
    ));

    // Направленный свет (солнце)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Тестовый куб по центру (0,0,0)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.2, 0.6, 0.9)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

pub fn cleanup_editor_scene(
    mut commands: Commands,
    camera_q: Query<Entity, With<EditorCamera>>,
    light_q: Query<Entity, With<DirectionalLight>>,
    mesh_q: Query<Entity, With<Handle<Mesh>>>,
) {
    for entity in &camera_q {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &light_q {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &mesh_q {
        commands.entity(entity).despawn_recursive();
    }
}