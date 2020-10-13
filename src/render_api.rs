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
        &self,
        event_type: unity_native_plugin::graphics::GfxDeviceEventType,
        interfaces: &unity_native_plugin::interface::UnityInterfaces,
    );

    fn get_uses_reverse_z(&self) -> bool;

    fn draw_simple_triangles(
        &self,
        world_matrix: [f32; 16],
        triangle_count: i32,
        vertices_float3_byte4: &[f32],
    );

    fn begin_modify_texture(
        &self,
        texture_handle: Handle,
        texture_width: i32,
        texture_height: i32,
    ) -> TextureBuffer;

    fn end_modify_texture(
        &self,
        texture_handle: Handle,
        texture_width: i32,
        texture_height: i32,
        buffer: TextureBuffer,
    );

    fn begin_modify_vertex_buffer(&self, buffer_handle: Handle) -> VertexBuffer;

    fn end_modify_vertex_buffer(&self, buffer_handle: Handle);
}

pub fn create_render_api(
    api_type: unity_native_plugin::graphics::GfxRenderer,
) -> Option<Box<dyn RenderAPI>> {
    None
}
