use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

#[derive(Component)]
pub struct EditorGrid;

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 40.0;
    let step = 1.0;
    let half = size / 2.0;

    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut idx = 0;

    let mut current = -half;
    while current <= half {
        // Линии вдоль Z
        positions.push([current, 0.0, -half]);
        positions.push([current, 0.0, half]);
        indices.push(idx);
        indices.push(idx + 1);
        idx += 2;

        // Линии вдоль X
        positions.push([-half, 0.0, current]);
        positions.push([half, 0.0, current]);
        indices.push(idx);
        indices.push(idx + 1);
        idx += 2;

        current += step;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.5, 0.5, 0.5, 0.4),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        },
        EditorGrid,
    ));
}