#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec3 v_VoxelSpace;

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

layout(set = 1, binding = 0) uniform uvec3 size;

void main() {
    v_Uv = Vertex_Uv;
    v_VoxelSpace = Vertex_Position + size / 2.0;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
