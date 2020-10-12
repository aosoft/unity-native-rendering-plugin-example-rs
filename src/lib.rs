mod render_api;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        let graphics = interfaces.interface::<unity_native_plugin::graphics::UnityGraphics>().unwrap();
        graphics.register_device_event_callback(Some(on_grapihcs_device_event));
        on_grapihcs_device_event(unity_native_plugin::graphics::GfxDeviceEventType::Initialize);
    }
    fn unity_plugin_unload() {
    }
}

static mut time: f32 = 0.0;

#[allow(non_snake_case)]
extern "system" fn SetTimeFromUnity(t: f32) {
    unsafe {
        time = t;
    }
}

static mut texture_handle: render_api::Handle = std::ptr::null_mut();
static mut texture_width: i32 = 0;
static mut texture_height: i32 = 0;

#[allow(non_snake_case)]
extern "system" fn SetTextureFromUnity(handle: render_api::Handle, w: i32, h: i32) {
    unsafe {
        texture_handle = handle;
        texture_width = w;
        texture_height = h;
    }
}

static mut vertex_buffer_handle: render_api::Handle = std::ptr::null_mut();
static mut vertex_buffer_vertex_count: i32 = 0;

#[repr(C)]
struct MeshVertex {
    pos: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
    uv: [f32; 2],
}

static mut vertex_source: Vec<MeshVertex> = Vec::<MeshVertex>::new();

#[allow(non_snake_case)]
extern "system" fn SetMeshBuffersFromUnity(
    handle: render_api::Handle,
    vertex_count: i32,
    source_vertices: *const f32,
    source_normals: *const f32,
    source_uv: *const f32,
) {
    unsafe {
        vertex_buffer_handle = handle;
        vertex_buffer_vertex_count = vertex_count;

        vertex_source = Vec::<MeshVertex>::with_capacity(vertex_count as usize);
        let mut source_vertices = source_vertices;
        let mut source_normals = source_normals;
        let mut source_uv = source_uv;
        for i in 0..vertex_count {
            let vertex = MeshVertex {
                pos: [*source_vertices, *source_vertices.offset(1), *source_vertices.offset(2)],
                normal: [*source_normals, *source_normals.offset(1), *source_normals.offset(2)],
                color: [0.0; 4],
                uv: [*source_uv, *source_uv.offset(1)]
            };
            source_vertices = source_vertices.offset(3);
            source_normals = source_normals.offset(3);
            source_uv = source_uv.offset(2);
        }
    }
}

extern "system" fn on_grapihcs_device_event(event_type: unity_native_plugin::graphics::GfxDeviceEventType) {

}
