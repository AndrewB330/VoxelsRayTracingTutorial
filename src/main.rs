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
                    // Some arbitrary coloring scheme.
                    let index = (x * grid_size * grid_size + y * grid_size + z) as usize;
                    if dx + dy + dz <= (radius as f32 * 1.4) as u32 {
                        v[index] = 0xff6b21;
                    } else {
                        v[index] = 0xffc821;
                    }
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

    let (model, size) = model("assets/models/station-draft.vox");
    let data = voxels_to_data(model);

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: &data,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            size.x as f32,
            size.y as f32,
            size.z as f32,
        ))),
        material: voxel_materials.add(VoxelVolumeMaterial {
            size,
            voxel_data: buffer,
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::splat(1.0 / size.y as f32)),
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
        transform: Transform::from_xyz(-1.0, 0.6, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        camera.look_at(Vec3::new(0.0, 0.2, 0.0), Vec3::Y);
    }
}

// Load .vox format model.
fn model(file: &str) -> (Vec<u32>, UVec3) {
    let vox_data = vox_format::from_file(file).unwrap();
    let size = vox_data.models[0].size;
    let mut res = vec![0; (size.x * size.y * size.z) as usize];

    for x in 1..size.x - 1 {
        for y in 1..size.y - 1 {
            for z in 1..size.z - 2 {
                if let Some(v) = vox_data.models[0].get_voxel([x as i8, y as i8, z as i8].into()) {
                    let index = (x * size.y * size.z + z * size.y + y) as usize;
                    let color = vox_data.palette.colors[v.color_index.0 as usize];
                    let r = color.r as u32;
                    let g = color.g as u32;
                    let b = color.b as u32;
                    let hex = (r << 16) | (g << 8) | b;
                    res[index] = hex;
                }
            }
        }
    }

    (res, UVec3::new(size.x, size.z, size.y))
}
