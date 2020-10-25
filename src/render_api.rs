pub type Handle = *mut std::ffi::c_void;

pub trait TextureBuffer {
    unsafe fn ptr(&self) -> *const std::ffi::c_void;
    unsafe fn mut_ptr(&mut self) -> *mut std::ffi::c_void;
    fn row_pitch(&self) -> i32;
}

pub trait VertexBuffer {
    unsafe fn ptr(&self) -> *const std::ffi::c_void;
    unsafe fn mut_ptr(&mut self) -> *mut std::ffi::c_void;
    fn size(&self) -> i32;
}

#[repr(C)]
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
    ) -> Box<dyn TextureBuffer>;

    fn end_modify_texture(
        &self,
        texture_handle: Handle,
        texture_width: i32,
        texture_height: i32,
        buffer: Box<dyn TextureBuffer>,
    );

    fn begin_modify_vertex_buffer(&self, buffer_handle: Handle) -> Box<dyn VertexBuffer>;

    fn end_modify_vertex_buffer(&self, buffer_handle: Handle);
}

pub fn create_render_api(
    api_type: unity_native_plugin::graphics::GfxRenderer,
) -> Option<Box<dyn RenderAPI>> {
    None
}
