use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::mesh::MeshVertexBufferLayout,
    render::render_resource::*,
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "3ed3d2a6-0117-4cd9-a517-ffc4312b574f"]
pub struct VoxelVolumeMaterial {}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for VoxelVolumeMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/voxel_volume_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/voxel_volume_material.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if descriptor.label == Some("opaque_mesh_pipeline".into()) {
            descriptor.vertex.entry_point = "main".into();
            descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        }
        Ok(())
    }
}
