use winapi::_core::ffi::c_void;
use crate::render_api::{MyVertex, VertexBuffer, TextureBuffer};
use unity_native_plugin::interface::UnityInterfaces;
use unity_native_plugin::graphics::GfxDeviceEventType;

pub struct RenderAPID3D11
{

}

impl Drop for RenderAPID3D11 {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl crate::render_api::RenderAPI for RenderAPID3D11 {
    fn process_device_event(&self, event_type: GfxDeviceEventType, interfaces: &UnityInterfaces) {
        unimplemented!()
    }

    fn get_uses_reverse_z(&self) -> bool {
        unimplemented!()
    }

    fn draw_simple_triangles(&self, world_matrix: [f32; 16], triangle_count: i32, vertices_float3_byte4: &[MyVertex]) {
        unimplemented!()
    }

    fn begin_modify_texture(&self, texture_handle: *mut c_void, texture_width: i32, texture_height: i32) -> TextureBuffer {
        unimplemented!()
    }

    fn end_modify_texture(&self, texture_handle: *mut c_void, texture_width: i32, texture_height: i32, buffer: TextureBuffer) {
        unimplemented!()
    }

    fn begin_modify_vertex_buffer(&self, buffer_handle: *mut c_void) -> VertexBuffer {
        unimplemented!()
    }

    fn end_modify_vertex_buffer(&self, buffer_handle: *mut c_void) {
        unimplemented!()
    }
}