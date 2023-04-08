use bevy::{
    prelude::*,
    render::{
        render_resource::{BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
    },
};

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

pub fn create_sphere_voxels(grid_size: u32, radius: u32) -> Vec<u32> {
    let mut v = vec![0; (grid_size * grid_size * grid_size) as usize];

    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                let dx = x.abs_diff(grid_size / 2);
                let dy = y.abs_diff(grid_size / 2);
                let dz = z.abs_diff(grid_size / 2);
                if dx * dx + dy * dy + dz * dz < radius * radius {
                    v[(x * grid_size * grid_size + y * grid_size + z) as usize] = 0xFFFFFF;
                }
            }
        }
    }

    v
}

pub fn voxels_to_data(volume: Vec<u32>) -> Vec<u8> {
    let mut data = vec![];

    for val in volume {
        data.append(&mut val.to_le_bytes().to_vec());
    }

    data
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_materials: ResMut<Assets<VoxelVolumeMaterial>>,
    render_device: Res<RenderDevice>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let size = 16;
    let sphere = create_sphere_voxels(size, 7);
    let data = voxels_to_data(sphere);

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: &data,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: size as f32 })),
        material: voxel_materials.add(VoxelVolumeMaterial {
            size: UVec3::splat(size),
            voxel_data: buffer,
        }),
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
