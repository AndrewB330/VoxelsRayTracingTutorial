use bevy::prelude::*;

mod voxel_volume_material;
use voxel_volume_material::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(MaterialPlugin::<VoxelVolumeMaterial>::default())
        .add_startup_system(setup)
        .add_system(rotate_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_materials: ResMut<Assets<VoxelVolumeMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let size = 16;

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: size as f32 })),
        material: voxel_materials.add(VoxelVolumeMaterial { size: UVec3::splat(size) }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::splat(1.0 / size as f32)),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -5.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 0.05, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_camera(
    mut cameras: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut pause: Local<bool>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *pause = !*pause;
    }

    if *pause {
        return;
    }

    for mut camera in cameras.iter_mut() {
        camera.translation = Quat::from_rotation_y(0.1 * time.delta_seconds()) * camera.translation;
        camera.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
    }
}
