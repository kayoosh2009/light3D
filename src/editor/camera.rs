use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Component)]
pub struct EditorCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub move_speed: f32,
    pub sensitivity: f32,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self {
            pitch: -0.52, // Около -30 градусов наклон вниз
            yaw: -0.78,   // -45 градусов поворот вокруг Y
            move_speed: 12.0,
            sensitivity: 0.003,
        }
    }
}

pub fn spawn_editor_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Точка фокуса — центр куба
    let target = Vec3::new(0.0, 0.5, 0.0);
    let cam_pos = Vec3::new(7.0, 5.5, 7.0);
    let look_transform = Transform::from_translation(cam_pos).looking_at(target, Vec3::Y);
    let (yaw, pitch, _) = look_transform.rotation.to_euler(EulerRot::YXZ);

    let cam_settings = EditorCamera {
        pitch,
        yaw,
        move_speed: 12.0,
        sensitivity: 0.003,
    };

    commands.spawn((
        Camera3dBundle {
            transform: look_transform,
            ..default()
        },
        cam_settings,
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

pub fn camera_controller_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut windows: Query<&mut Window>,
    mut camera_query: Query<(&mut Transform, &mut EditorCamera)>,
) {
    let Ok((mut transform, mut cam)) = camera_query.get_single_mut() else {
        return;
    };
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    let rmb_held = mouse_button.pressed(MouseButton::Right);

    // Курсор: прячем и лочим только при зажатой ПКМ
    if mouse_button.just_pressed(MouseButton::Right) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
    if mouse_button.just_released(MouseButton::Right) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    // Вращение камеры мышью
    if rmb_held {
        for motion in mouse_motion_events.read() {
            cam.yaw -= motion.delta.x * cam.sensitivity;
            cam.pitch -= motion.delta.y * cam.sensitivity;
            cam.pitch = cam.pitch.clamp(-1.54, 1.54); // Ограничение поворота вверх/вниз
        }
        transform.rotation = Quat::from_euler(EulerRot::YXZ, cam.yaw, cam.pitch, 0.0);
    } else {
        mouse_motion_events.clear();
    }

    // Зум / приближение колесиком мыши
    for ev in mouse_wheel_events.read() {
        let forward = transform.forward();
        transform.translation += forward * ev.y * 1.5;
    }

    // Перемещение клавишами (WASD + Q/E)
    let mut speed = cam.move_speed;
    if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        speed *= 2.5; // Ускорение при Shift
    }

    let mut direction = Vec3::ZERO;
    let forward = transform.forward();
    let right = transform.right();
    let up = Vec3::Y;

    if keys.pressed(KeyCode::KeyW) {
        direction += *forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= *forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *right;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= *right;
    }
    if keys.pressed(KeyCode::KeyE) {
        direction += up;
    }
    if keys.pressed(KeyCode::KeyQ) {
        direction -= up;
    }

    if direction.length_squared() > 0.0 {
        transform.translation += direction.normalize() * speed * time.delta_seconds();
    }
}