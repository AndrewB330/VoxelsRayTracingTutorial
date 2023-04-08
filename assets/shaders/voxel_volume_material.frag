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

void main() {
    o_Color = vec4(v_VoxelSpace % vec3(1.0), 1.0);
}
