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
const float EPS = 1e-5;
const vec3 LIGHT_DIR = vec3(0.1, -0.5, 0.3);

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
    vec3 ray_direction_inv = 1.0 / ray_direction;
    vec3 next_cell_delta = 0.5 + sign(ray_direction) * (0.5 + EPS);

    ray_point += ray_direction * EPS;

    ivec3 cell = ivec3(floor(ray_point));
    vec3 time = vec3(0.0);

    bool hit = false;

    // Primitive raytracing - fixed length steps along the ray.
    for (int i = 0; i < 128 && checkBoundaries(cell); i++) {
        if (hit = getVoxel(cell) != 0) break;

        // Calculate time until next intersection per component.
        time = (cell + next_cell_delta - ray_point) * ray_direction_inv;
        ray_point += ray_direction * min(min(time.x, time.y), time.z);
        cell = ivec3(floor(ray_point));
    }

    if (!hit) {
        // We did not find any hit - discard this fragment.
        discard;
    }

    uint color_hex = getVoxel(cell);
    float r = ((color_hex & 0xFF0000) >> 16) / 255.0;
    float g = ((color_hex & 0x00FF00) >>  8) / 255.0;
    float b = ((color_hex & 0x0000FF) >>  0) / 255.0;
    vec3 color = vec3(r, g, b);
    vec3 normal = -step(time.xyz, time.yzx) * step(time.xyz, time.zxy) * sign(ray_direction);

    float n_dot_l = clamp(dot(normal, -normalize(LIGHT_DIR)), 0.0, 1.0);
    float intensity = mix(0.2, 1.0, n_dot_l);
    o_Color = vec4(intensity * color, 1.0);
}
