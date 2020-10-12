pub type Handle = *mut std::ffi::c_void;

pub struct TextureBuffer {
    pub ptr: *mut std::ffi::c_void,
    pub row_pitch: i32,
}

pub struct VertexBuffer {
    pub ptr: *mut std::ffi::c_void,
    pub size: i32,
}

pub trait RenderAPI {
    fn process_device_event(
        event_type: unity_native_plugin::graphics::GfxDeviceEventType,
        interfaces: unity_native_plugin::IUnityInterfaces,
    );

    fn get_uses_reverse_z() -> bool;

    fn draw_simple_triangles(
        world_matrix: [f32; 16],
        triangle_count: i32,
        vertices_float3_byte4: &[f32],
    );

    fn begin_modify_texture(
        texture_handle: Handle,
        texture_width: i32,
        texture_height: i32,
    ) -> TextureBuffer;

    fn end_modify_texture(
        texture_handle: Handle,
        texture_width: i32,
        texture_height: i32,
        buffer: TextureBuffer,
    );

    fn begin_modify_vertex_buffer(buffer_handle: Handle) -> VertexBuffer;

    fn end_modify_vertex_buffer(buffer_handle: Handle);
}
