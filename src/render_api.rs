pub type Handle = *mut std::ffi::c_void;

pub struct TextureBuffer {
    pub ptr: *mut std::ffi::c_void,
    pub row_pitch: i32,
}

pub struct VertexBuffer {
    pub ptr: *mut std::ffi::c_void,
    pub size: i32,
}

pub struct MyVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: u32,
}

pub trait RenderAPI: Drop {
    fn process_device_event(
        &mut self,
        event_type: unity_native_plugin::graphics::GfxDeviceEventType,
        interfaces: &unity_native_plugin::interface::UnityInterfaces,
    );

    fn get_uses_reverse_z(&self) -> bool;

    fn draw_simple_triangles(
        &self,
        world_matrix: [f32; 16],
        triangle_count: i32,
        vertices_float3_byte4: &[MyVertex],
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
