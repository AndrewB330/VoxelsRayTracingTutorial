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

struct HitInfo {
    vec3 pos;
    vec3 color;
    ivec3 cell;
    ivec3 local_normal;
};

const ivec3 ZERO = ivec3(0.0);
const float EPS = 5e-5;
const vec3 LIGHT_DIR = vec3(0.1, -0.5, 0.3);

bool checkBoundaries(ivec3 cell) {
    // Check boundaries:
    // 1. All voxel space coordinate components should be less than corresponding size component.
    // 2. All voxel space coordinate components should be greater or equal than zero.
    return all(lessThan(cell, size)) && all(greaterThanEqual(cell, ZERO));
}

float computeAo(HitInfo info) {
    ivec3 cell = info.cell + info.local_normal;

    vec3 mask = abs(info.local_normal);
    vec2 uv = vec2(dot(mask * info.pos.yzx, vec3(1.0)), dot(mask * info.pos.zxy, vec3(1.0))) % vec2(1.0);

    ivec3 dir1 = ivec3((step(0.5, uv.x) * 2.0 - 1.0) * mask.zxy);
    ivec3 dir2 = ivec3((step(0.5, uv.y) * 2.0 - 1.0) * mask.yzx);
    
    float side1 =  float(checkBoundaries(cell + dir1) && getVoxel(cell + dir1) != 0u);
    float side2 =  float(checkBoundaries(cell + dir2) && getVoxel(cell + dir2) != 0u);
    float corner = float(checkBoundaries(cell + dir1 + dir2) && getVoxel(cell + dir1 + dir2) != 0u);

    vec4 ambient = 1.0 - vec4(0.0, side1 * 0.5, (side1 + side2 + max(side1*side2, corner)) * 0.25, side2 * 0.5);
    
    vec2 corner_uv = abs(uv - 0.5) * 2.0;
    float interpAo = mix(mix(ambient.x, ambient.y, corner_uv.x), mix(ambient.w, ambient.z, corner_uv.x), corner_uv.y);
    return pow(interpAo, 0.5);
}

bool trace(vec3 origin, vec3 direction, out HitInfo info) {
    vec3 direction_inv = 1.0 / direction;
    vec3 next_cell_delta = 0.5 + sign(direction) * (0.5 + EPS);

    origin += direction * EPS;

    ivec3 cell = ivec3(floor(origin));
    vec3 time = vec3(0.0);

    bool hit = false;

    // Primitive raytracing - fixed length steps along the ray.
    for (int i = 0; i < 256 && checkBoundaries(cell); i++) {
        if (hit = getVoxel(cell) != 0) break;

        // Calculate time until next intersection per component.
        time = (cell + next_cell_delta - origin) * direction_inv;
        origin += direction * min(min(time.x, time.y), time.z);
        cell = ivec3(floor(origin));
    }

    if (!hit) {
        return false;
    }

    uint color_hex = getVoxel(cell);
    float r = ((color_hex & 0xFF0000) >> 16) / 255.0;
    float g = ((color_hex & 0x00FF00) >>  8) / 255.0;
    float b = ((color_hex & 0x0000FF) >>  0) / 255.0;
    info.color = vec3(r, g, b);
    info.local_normal = ivec3(-step(time.xyz, time.yzx) * step(time.xyz, time.zxy) * sign(direction));
    info.pos = origin;
    info.cell = cell;
    return true;
}

void main() {
    vec3 ray_origin = (transpose(InverseTransposeModel) * InverseView * vec4(0.0, 0.0, 0.0, 1.0)).xyz + size / 2.0;
    vec3 ray_point = v_VoxelSpace;
    vec3 ray_direction = normalize(ray_point - ray_origin);
    
    HitInfo info;
    if (!trace(ray_point, ray_direction, info)) {
        discard;
    }

    // Direction to light source in global coordinates.
    vec3 l = -normalize(LIGHT_DIR);
    // Normal in global coordinates.
    vec3 n = normalize((InverseTransposeModel * vec4(info.local_normal, 0.0)).xyz);

    // Light intensity from 0.2 to 1.0.
    float light_intensity = mix(0.2, 1.0, max(dot(n, l), 0.0));
    // Approximate ambient occlusion, 0.0 - fully occluded, 1.0 - not occluded.
    float ambient_occlusion = computeAo(info);
    // Voxel color.
    vec3 color = info.color;

    // Shadow ray.
    if (trace(info.pos + n * EPS, l, info)) {
        light_intensity = 0.2;
    }

    o_Color = vec4(light_intensity * ambient_occlusion * color, 1.0);
}
