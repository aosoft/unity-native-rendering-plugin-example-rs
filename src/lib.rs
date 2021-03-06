mod render_api;

#[cfg(target_os = "windows")]
mod render_api_d3d11;

#[cfg(target_os = "windows")]
mod win_util;

#[cfg(target_feature = "vulkan")]
mod render_api_vulkan;

#[cfg(target_feature = "vulkan")]
mod vulkan_api;

static mut GRAPHICS: Option<unity_native_plugin::graphics::UnityGraphics> = None;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        unsafe {
            GRAPHICS = interfaces.interface::<unity_native_plugin::graphics::UnityGraphics>();
            if let Some(g) = &GRAPHICS {
                g.register_device_event_callback(Some(on_grapihcs_device_event));

                #[cfg(target_feature = "vulkan")]
                if g.renderer() == unity_native_plugin::graphics::GfxRenderer::Vulkan {
                    render_api_vulkan::on_plugin_load(interfaces);
                }
            }
        }
        on_grapihcs_device_event(unity_native_plugin::graphics::GfxDeviceEventType::Initialize);
    }
    fn unity_plugin_unload() {
        unsafe {
            if let Some(g) = &GRAPHICS {
                g.unregister_device_event_callback(Some(on_grapihcs_device_event));
            }
        }
    }
}

static mut TIME: f32 = 0.0;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetTimeFromUnity(t: f32) {
    unsafe {
        TIME = t;
    }
}

static mut TEXTURE_HANDLE: render_api::Handle = std::ptr::null_mut();
static mut TEXTURE_WIDTTH: i32 = 0;
static mut TEXTURE_HEIGHT: i32 = 0;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetTextureFromUnity(handle: render_api::Handle, w: i32, h: i32) {
    unsafe {
        TEXTURE_HANDLE = handle;
        TEXTURE_WIDTTH = w;
        TEXTURE_HEIGHT = h;
    }
}

static mut VERTEX_BUFFER_HANDLE: render_api::Handle = std::ptr::null_mut();
static mut VERTEX_BUFFER_VERTEX_COUNT: i32 = 0;

#[repr(C)]
struct MeshVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

static mut VERTEX_SOURCE: Vec<MeshVertex> = Vec::<MeshVertex>::new();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn SetMeshBuffersFromUnity(
    handle: render_api::Handle,
    vertex_count: i32,
    source_vertices: *const f32,
    source_normals: *const f32,
    source_uv: *const f32,
) {
    unsafe {
        VERTEX_BUFFER_HANDLE = handle;
        VERTEX_BUFFER_VERTEX_COUNT = vertex_count;

        VERTEX_SOURCE = Vec::<MeshVertex>::with_capacity(vertex_count as usize);
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

            VERTEX_SOURCE.push(vertex);
        }
    }
}

static mut CURRENT_API: Option<Box<dyn render_api::RenderAPI>> = None;
static mut DEVICE_TYPE: unity_native_plugin::graphics::GfxRenderer =
    unity_native_plugin::graphics::GfxRenderer::Null;

extern "system" fn on_grapihcs_device_event(
    event_type: unity_native_plugin::graphics::GfxDeviceEventType,
) {
    if event_type == unity_native_plugin::graphics::GfxDeviceEventType::Initialize {
        unsafe {
            DEVICE_TYPE = GRAPHICS.as_ref().unwrap().renderer();
            CURRENT_API = render_api::create_render_api(DEVICE_TYPE);
        }
    }

    unsafe {
        if let Some(api) = CURRENT_API.as_mut() {
            api.process_device_event(
                event_type,
                unity_native_plugin::interface::UnityInterfaces::get(),
            );
        }
    }

    if event_type == unity_native_plugin::graphics::GfxDeviceEventType::Shutdown {
        unsafe {
            DEVICE_TYPE = unity_native_plugin::graphics::GfxRenderer::Null;
            CURRENT_API = None;
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

    if let Some(api) = unsafe { CURRENT_API.as_ref() } {
        let phi = unsafe { TIME };
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let depth = 0.7;
        let final_depth = if api.get_uses_reverse_z() {
            1.0 - depth
        } else {
            depth
        };
        let world_matrix = [
            cos_phi,
            -sin_phi,
            0.0,
            0.0,
            sin_phi,
            cos_phi,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            final_depth,
            1.0,
        ];

        api.draw_simple_triangles(world_matrix, 1, &verts);
    }
}

fn modify_texture_pixels() {
    unsafe {
        let handle = TEXTURE_HANDLE;
        let width = TEXTURE_WIDTTH;
        let height = TEXTURE_HEIGHT;

        if handle.is_null() {
            return;
        }
        if let Some(api) = CURRENT_API.as_ref() {
            if let Some(mut buffer) = api.begin_modify_texture(handle, width, height) {
                if buffer.ptr().is_null() {
                    return;
                }

                let t = TIME * 4.0;

                let mut dst = buffer.mut_ptr() as *mut u8;
                for y in 0..height {
                    let mut ptr = dst;
                    for x in 0..width {
                        let vv: i32 = ((127.0 + (127.0 * (x as f32 / 7.0 + t).sin()))
                            + (127.0 + (127.0 * (y as f32 / 5.0 - t).sin()))
                            + (127.0 + (127.0 * ((x + y) as f32 / 6.0 - t).sin()))
                            + (127.0 + (127.0 * (((x * x + y * y) as f32).sqrt() / 4.0 - t).sin())))
                            as i32
                            / 4;
                        *ptr = vv as u8;
                        ptr = ptr.offset(1);
                        *ptr = vv as u8;
                        ptr = ptr.offset(1);
                        *ptr = vv as u8;
                        ptr = ptr.offset(1);
                        *ptr = vv as u8;
                        ptr = ptr.offset(1);
                    }

                    dst = dst.offset(buffer.row_pitch() as isize);
                }
                api.end_modify_texture(handle, width, height, buffer);
            }
        }
    }
}

fn modify_vertex_buffer() {
    unsafe {
        let handle = VERTEX_BUFFER_HANDLE;
        let vertex_count = VERTEX_BUFFER_VERTEX_COUNT;
        if let Some(api) = CURRENT_API.as_ref() {
            if let Some(mut buffer) = api.begin_modify_vertex_buffer(handle) {
                if buffer.ptr().is_null() {
                    return;
                }
                let vertex_stride = buffer.size() / vertex_count;
                let t = TIME * 3.0;

                let mut buffer_ptr = buffer.mut_ptr() as *mut u8;
                for i in 0..vertex_count {
                    let src = &VERTEX_SOURCE[i as usize];
                    let mut dst = &mut *(buffer_ptr as *mut MeshVertex);
                    dst.pos[0] = src.pos[0];
                    dst.pos[1] = src.pos[1]
                        + (src.pos[0] * 1.1 + t).sin() * 0.4
                        + (src.pos[2] * 0.9 - t).sin() * 0.3;
                    dst.pos[2] = src.pos[2];
                    dst.normal[0] = src.normal[0];
                    dst.normal[1] = src.normal[1];
                    dst.normal[2] = src.normal[2];
                    dst.uv[0] = src.uv[0];
                    dst.uv[1] = src.uv[1];

                    buffer_ptr = buffer_ptr.offset(vertex_stride as isize);
                }
            }
            api.end_modify_vertex_buffer(handle);
        }
    }
}

extern "system" fn on_render_event(_: std::os::raw::c_int) {
    if unsafe { CURRENT_API.is_none() } {
        return;
    }

    draw_colored_triangle();
    modify_texture_pixels();
    modify_vertex_buffer();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn GetRenderEventFunc() -> unity_native_plugin::graphics::RenderingEvent {
    Some(on_render_event)
}

#[test]
fn test_modify_texture_pixels() {
    let instant = std::time::Instant::now();
    unity_native_plugin_tester::d3d11::test_plugin_d3d11(
        (256, 256),
        |_window, context| {
            SetTextureFromUnity(
                context.back_buffer().as_raw() as _,
                context.back_buffer_desc().Width as _,
                context.back_buffer_desc().Height as _,
            );
        },
        |_window, _context| {
            SetTimeFromUnity(instant.elapsed().as_secs_f32());
            modify_texture_pixels();
            unity_native_plugin_tester::window::LoopResult::ContinueOnWindowEvent
        },
        |_, _| {},
        unity_plugin_load,
        unity_plugin_unload,
    )
    .unwrap();
}
