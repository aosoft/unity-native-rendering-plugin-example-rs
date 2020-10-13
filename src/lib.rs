mod render_api;

static mut graphics: Option<unity_native_plugin::graphics::UnityGraphics> = None;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        unsafe {
            graphics = interfaces.interface::<unity_native_plugin::graphics::UnityGraphics>();
            if let Some(g) = &graphics {
                g.register_device_event_callback(Some(on_grapihcs_device_event));
            }
        }
        on_grapihcs_device_event(unity_native_plugin::graphics::GfxDeviceEventType::Initialize);
    }
    fn unity_plugin_unload() {
        unsafe {
            if let Some(g) = &graphics {
                g.unregister_device_event_callback(Some(on_grapihcs_device_event));
            }
        }
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
        for _ in 0..vertex_count {
            let vertex = MeshVertex {
                pos: [
                    *source_vertices,
                    *source_vertices.offset(1),
                    *source_vertices.offset(2),
                ],
                normal: [
                    *source_normals,
                    *source_normals.offset(1),
                    *source_normals.offset(2),
                ],
                color: [0.0; 4],
                uv: [*source_uv, *source_uv.offset(1)],
            };
            source_vertices = source_vertices.offset(3);
            source_normals = source_normals.offset(3);
            source_uv = source_uv.offset(2);

            vertex_source.push(vertex);
        }
    }
}

static mut current_api: Option<Box<dyn render_api::RenderAPI>> = None;
static mut device_type: unity_native_plugin::graphics::GfxRenderer =
    unity_native_plugin::graphics::GfxRenderer::Null;

extern "system" fn on_grapihcs_device_event(
    event_type: unity_native_plugin::graphics::GfxDeviceEventType,
) {
    if event_type == unity_native_plugin::graphics::GfxDeviceEventType::Initialize {
        unsafe {
            device_type = graphics.as_ref().unwrap().renderer();
            current_api = render_api::create_render_api(device_type);
        }
    }

    unsafe {
        if let Some(api) = current_api.as_ref() {
            api.process_device_event(
                event_type,
                unity_native_plugin::interface::UnityInterfaces::get(),
            );
        }
    }

    if event_type == unity_native_plugin::graphics::GfxDeviceEventType::Shutdown {
        unsafe {
            device_type = unity_native_plugin::graphics::GfxRenderer::Null;
            current_api = None;
        }
    }
}

fn draw_colored_triangle() {
    let verts = [
        render_api::MyVertex {
            x: -0.5,
            y: -0.25,
            z: 0.0,
            color: 0xFFff0000,
        },
        render_api::MyVertex {
            x: 0.5,
            y: -0.25,
            z: 0.0,
            color: 0xFF00ff00,
        },
        render_api::MyVertex {
            x: 0.0,
            y: 0.5,
            z: 0.0,
            color: 0xFF0000ff,
        },
    ];

    if let Some(api) = unsafe { current_api.as_ref() } {
        let phi = unsafe { time };
        let cosPhi = phi.cos();
        let sinPhi = phi.sin();
        let depth = 0.7;
        let finalDepth = if api.get_uses_reverse_z() {
            1.0 - depth
        } else {
            depth
        };
        let worldMatrix = [
            cosPhi, -sinPhi, 0.0, 0.0, sinPhi, cosPhi, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            finalDepth, 1.0,
        ];

        api.draw_simple_triangles(worldMatrix, 1, &verts);
    }
}
