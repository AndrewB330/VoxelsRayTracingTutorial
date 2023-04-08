#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec3 v_VoxelSpace;

layout(location = 0) out vec4 o_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
    mat4 InverseTransposeModel;
    uint flags;
};

layout(set = 1, binding = 0) uniform ivec3 size;

layout(set = 1, binding = 1) buffer VolumeData {
    uint data[];
};

uint getVoxel(ivec3 cell) {
    return data[cell.x * size.y * size.z + cell.y * size.z + cell.z];
}

const ivec3 ZERO = ivec3(0.0);

bool checkBoundaries(ivec3 cell) {
    // Check boundaries:
    // 1. All voxel space coordinate components should be less than corresponding size component.
    // 2. All voxel space coordinate components should be greater or equal than zero.
    return all(lessThan(cell, size)) && all(greaterThanEqual(cell, ZERO));
}

void main() {
    vec3 ray_origin = (transpose(InverseTransposeModel) * InverseView * vec4(0.0, 0.0, 0.0, 1.0)).xyz + size / 2.0;
    vec3 ray_point = v_VoxelSpace;
    vec3 ray_direction = normalize(ray_point - ray_origin);

    ivec3 cell = ivec3(floor(ray_point));
    ivec3 previous_cell = cell;

    // Primitive raytracing - fixed length steps along the ray.
    for (int i = 0; i < 128; i++) {
        if (checkBoundaries(cell) && getVoxel(cell) != 0) {
            // Hit! Use normal apprxoimation as a color.
            ivec3 normal = abs(previous_cell - cell);
            o_Color = vec4( /* Color: */ vec3(normal), /* Alpha: */ 1.0);
            return;
        }

        ray_point += ray_direction * 0.1;
        previous_cell = cell;
        cell = ivec3(floor(ray_point));
    }

    // We did not find any hit - discard this fragment.
    discard;
}
