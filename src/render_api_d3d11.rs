use crate::render_api::{MyVertex, RenderAPI, TextureBuffer, VertexBuffer};
use unity_native_plugin::graphics::GfxDeviceEventType;
use unity_native_plugin::interface::UnityInterfaces;
use winapi::_core::ffi::c_void;
use winapi::shared::dxgiformat::*;
use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;
use winapi::um::d3d11::*;
use wio::com::ComPtr;

pub struct RenderAPID3D11 {
    device: Option<ComPtr<ID3D11Device>>,
    vb: Option<ComPtr<ID3D11Buffer>>,
    cb: Option<ComPtr<ID3D11Buffer>>,
    vertex_shader: Option<ComPtr<ID3D11VertexShader>>,
    pixel_shader: Option<ComPtr<ID3D11PixelShader>>,
    input_layout: Option<ComPtr<ID3D11InputLayout>>,
    rasterizer_state: Option<ComPtr<ID3D11RasterizerState>>,
    blend_state: Option<ComPtr<ID3D11BlendState>>,
    depth_stencil_state: Option<ComPtr<ID3D11DepthStencilState>>,
}

#[inline]
fn check_hr(hr: HRESULT) -> Result<(), HRESULT> {
    if SUCCEEDED(hr) {
        Ok(())
    } else {
        Err(hr)
    }
}

impl Drop for RenderAPID3D11 {
    fn drop(&mut self) {}
}

impl crate::render_api::RenderAPI for RenderAPID3D11 {
    fn process_device_event(
        &mut self,
        event_type: GfxDeviceEventType,
        interfaces: &UnityInterfaces,
    ) {
        match (event_type) {
            GfxDeviceEventType::Initialize => {
                let intf = interfaces.interface::<unity_native_plugin::d3d11::UnityGraphicsD3D11>();
                self.device =
                    unsafe { Some(ComPtr::from_raw(intf.unwrap().device() as *mut ID3D11Device)) };
                unsafe { self.device.as_ref().unwrap().AddRef() };
                self.create_resources();
            }
            _ => {}
        }
    }

    fn get_uses_reverse_z(&self) -> bool {
        unimplemented!()
    }

    fn draw_simple_triangles(
        &self,
        world_matrix: [f32; 16],
        triangle_count: i32,
        vertices_float3_byte4: &[MyVertex],
    ) {
        unimplemented!()
    }

    fn begin_modify_texture(
        &self,
        texture_handle: *mut c_void,
        texture_width: i32,
        texture_height: i32,
    ) -> TextureBuffer {
        unimplemented!()
    }

    fn end_modify_texture(
        &self,
        texture_handle: *mut c_void,
        texture_width: i32,
        texture_height: i32,
        buffer: TextureBuffer,
    ) {
        unimplemented!()
    }

    fn begin_modify_vertex_buffer(&self, buffer_handle: *mut c_void) -> VertexBuffer {
        unimplemented!()
    }

    fn end_modify_vertex_buffer(&self, buffer_handle: *mut c_void) {
        unimplemented!()
    }
}

impl RenderAPID3D11 {
    pub fn new() -> Box<RenderAPID3D11> {
        Box::new(RenderAPID3D11 {
            device: None,
            vb: None,
            cb: None,
            vertex_shader: None,
            pixel_shader: None,
            input_layout: None,
            rasterizer_state: None,
            blend_state: None,
            depth_stencil_state: None,
        })
    }

    fn create_resources(&mut self) -> Result<(), HRESULT> {
        if let Some(device) = &self.device {
            unsafe {
                let desc = D3D11_BUFFER_DESC {
                    Usage: D3D11_USAGE_DEFAULT,
                    ByteWidth: 1024,
                    BindFlags: D3D11_BIND_VERTEX_BUFFER,
                    ..std::mem::zeroed()
                };

                let mut buffer: *mut ID3D11Buffer = std::ptr::null_mut();
                check_hr(device.CreateBuffer(&desc, std::ptr::null(), &mut buffer as _))?;
                self.vb = Some(ComPtr::from_raw(buffer));

                let desc = D3D11_BUFFER_DESC {
                    Usage: D3D11_USAGE_DEFAULT,
                    ByteWidth: 64,
                    BindFlags: D3D11_BIND_CONSTANT_BUFFER,
                    CPUAccessFlags: 0,
                    ..std::mem::zeroed()
                };

                let mut buffer: *mut ID3D11Buffer = std::ptr::null_mut();
                check_hr(device.CreateBuffer(&desc, std::ptr::null(), &mut buffer as _))?;
                self.cb = Some(ComPtr::from_raw(buffer));

                let mut shader: *mut ID3D11VertexShader = std::ptr::null_mut();
                check_hr(device.CreateVertexShader(
                    VERTEX_SHADER_CODE.as_ptr() as _,
                    VERTEX_SHADER_CODE.len(),
                    std::ptr::null_mut(),
                    &mut shader,
                ))?;
                self.vertex_shader = Some(ComPtr::from_raw(shader));

                let mut shader: *mut ID3D11PixelShader = std::ptr::null_mut();
                check_hr(device.CreatePixelShader(
                    PIXEL_SHADER_CODE.as_ptr() as _,
                    PIXEL_SHADER_CODE.len(),
                    std::ptr::null_mut(),
                    &mut shader,
                ))?;
                self.pixel_shader = Some(ComPtr::from_raw(shader));

                if let Some(vs) = &self.vertex_shader {
                    let desc = [
                        D3D11_INPUT_ELEMENT_DESC {
                            SemanticName: "POSITION\0".as_ptr() as _,
                            SemanticIndex: 0,
                            Format: DXGI_FORMAT_R32G32B32_FLOAT,
                            InputSlot: 0,
                            AlignedByteOffset: 0,
                            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                            InstanceDataStepRate: 0,
                        },
                        D3D11_INPUT_ELEMENT_DESC {
                            SemanticName: "COLOR\0".as_ptr() as _,
                            SemanticIndex: 0,
                            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                            InputSlot: 0,
                            AlignedByteOffset: 12,
                            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                            InstanceDataStepRate: 0,
                        },
                    ];

                    let mut layout: *mut ID3D11InputLayout = std::ptr::null_mut();
                    check_hr(device.CreateInputLayout(
                        desc.as_ptr(),
                        desc.len() as _,
                        VERTEX_SHADER_CODE.as_ptr() as _,
                        VERTEX_SHADER_CODE.len(),
                        &mut layout,
                    ))?;
                    self.input_layout = Some(ComPtr::from_raw(layout));

                    let desc = D3D11_RASTERIZER_DESC {
                        FillMode: D3D11_FILL_SOLID,
                        CullMode: D3D11_CULL_NONE,
                        DepthClipEnable: TRUE,
                        ..std::mem::zeroed()
                    };
                    let mut rs: *mut ID3D11RasterizerState = std::ptr::null_mut();
                    check_hr(device.CreateRasterizerState(&desc, &mut rs))?;
                    self.rasterizer_state = Some(ComPtr::from_raw(rs));

                    let desc = D3D11_DEPTH_STENCIL_DESC {
                        DepthEnable: TRUE,
                        DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ZERO,
                        DepthFunc: if self.get_uses_reverse_z() {
                            D3D11_COMPARISON_GREATER_EQUAL
                        } else {
                            D3D11_COMPARISON_LESS_EQUAL
                        },
                        ..std::mem::zeroed()
                    };
                    let mut ds: *mut ID3D11DepthStencilState = std::ptr::null_mut();
                    check_hr(device.CreateDepthStencilState(&desc, &mut ds))?;
                    self.depth_stencil_state = Some(ComPtr::from_raw(ds));

                    let mut desc: D3D11_BLEND_DESC = std::mem::zeroed();
                    desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                        BlendEnable: FALSE,
                        RenderTargetWriteMask: 0xf,
                        ..std::mem::zeroed()
                    };
                    let mut bs: *mut ID3D11BlendState = std::ptr::null_mut();
                    check_hr(device.CreateBlendState(&desc, &mut bs))?;
                    self.blend_state = Some(ComPtr::from_raw(bs));
                }
            }
            Ok(())
        } else {
            Err(S_FALSE)
        }
    }
}

static VERTEX_SHADER_CODE: [u8; 680] = [
    68, 88, 66, 67, 86, 189, 21, 50, 166, 106, 171, 1, 10, 62, 115, 48, 224, 137, 163, 129, 1, 0,
    0, 0, 168, 2, 0, 0, 4, 0, 0, 0, 48, 0, 0, 0, 0, 1, 0, 0, 4, 2, 0, 0, 84, 2, 0, 0, 65, 111, 110,
    57, 200, 0, 0, 0, 200, 0, 0, 0, 0, 2, 254, 255, 148, 0, 0, 0, 52, 0, 0, 0, 1, 0, 36, 0, 0, 0,
    48, 0, 0, 0, 48, 0, 0, 0, 36, 0, 1, 0, 48, 0, 0, 0, 0, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 2, 254, 255, 31, 0, 0, 2, 5, 0, 0, 128, 0, 0, 15, 144, 31, 0, 0, 2, 5, 0, 1, 128, 1, 0, 15,
    144, 5, 0, 0, 3, 0, 0, 15, 128, 0, 0, 85, 144, 2, 0, 228, 160, 4, 0, 0, 4, 0, 0, 15, 128, 1, 0,
    228, 160, 0, 0, 0, 144, 0, 0, 228, 128, 4, 0, 0, 4, 0, 0, 15, 128, 3, 0, 228, 160, 0, 0, 170,
    144, 0, 0, 228, 128, 2, 0, 0, 3, 0, 0, 15, 128, 0, 0, 228, 128, 4, 0, 228, 160, 4, 0, 0, 4, 0,
    0, 3, 192, 0, 0, 255, 128, 0, 0, 228, 160, 0, 0, 228, 128, 1, 0, 0, 2, 0, 0, 12, 192, 0, 0,
    228, 128, 1, 0, 0, 2, 0, 0, 15, 224, 1, 0, 228, 144, 255, 255, 0, 0, 83, 72, 68, 82, 252, 0, 0,
    0, 64, 0, 1, 0, 63, 0, 0, 0, 89, 0, 0, 4, 70, 142, 32, 0, 0, 0, 0, 0, 4, 0, 0, 0, 95, 0, 0, 3,
    114, 16, 16, 0, 0, 0, 0, 0, 95, 0, 0, 3, 242, 16, 16, 0, 1, 0, 0, 0, 101, 0, 0, 3, 242, 32, 16,
    0, 0, 0, 0, 0, 103, 0, 0, 4, 242, 32, 16, 0, 1, 0, 0, 0, 1, 0, 0, 0, 104, 0, 0, 2, 1, 0, 0, 0,
    54, 0, 0, 5, 242, 32, 16, 0, 0, 0, 0, 0, 70, 30, 16, 0, 1, 0, 0, 0, 56, 0, 0, 8, 242, 0, 16, 0,
    0, 0, 0, 0, 86, 21, 16, 0, 0, 0, 0, 0, 70, 142, 32, 0, 0, 0, 0, 0, 1, 0, 0, 0, 50, 0, 0, 10,
    242, 0, 16, 0, 0, 0, 0, 0, 70, 142, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 16, 16, 0, 0, 0, 0, 0,
    70, 14, 16, 0, 0, 0, 0, 0, 50, 0, 0, 10, 242, 0, 16, 0, 0, 0, 0, 0, 70, 142, 32, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 166, 26, 16, 0, 0, 0, 0, 0, 70, 14, 16, 0, 0, 0, 0, 0, 0, 0, 0, 8, 242, 32, 16, 0,
    1, 0, 0, 0, 70, 14, 16, 0, 0, 0, 0, 0, 70, 142, 32, 0, 0, 0, 0, 0, 3, 0, 0, 0, 62, 0, 0, 1, 73,
    83, 71, 78, 72, 0, 0, 0, 2, 0, 0, 0, 8, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 0, 0, 0, 7, 7, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 15, 15,
    0, 0, 80, 79, 83, 73, 84, 73, 79, 78, 0, 67, 79, 76, 79, 82, 0, 171, 79, 83, 71, 78, 76, 0, 0,
    0, 2, 0, 0, 0, 8, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 15, 0,
    0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 15, 0, 0, 0, 67, 79, 76, 79,
    82, 0, 83, 86, 95, 80, 111, 115, 105, 116, 105, 111, 110, 0, 171, 171,
];

static PIXEL_SHADER_CODE: [u8; 288] = [
    68, 88, 66, 67, 196, 65, 213, 199, 14, 78, 29, 150, 87, 236, 231, 156, 203, 125, 244, 112, 1,
    0, 0, 0, 32, 1, 0, 0, 4, 0, 0, 0, 48, 0, 0, 0, 124, 0, 0, 0, 188, 0, 0, 0, 236, 0, 0, 0, 65,
    111, 110, 57, 68, 0, 0, 0, 68, 0, 0, 0, 0, 2, 255, 255, 32, 0, 0, 0, 36, 0, 0, 0, 0, 0, 36, 0,
    0, 0, 36, 0, 0, 0, 36, 0, 0, 0, 36, 0, 0, 0, 36, 0, 1, 2, 255, 255, 31, 0, 0, 2, 0, 0, 0, 128,
    0, 0, 15, 176, 1, 0, 0, 2, 0, 8, 15, 128, 0, 0, 228, 176, 255, 255, 0, 0, 83, 72, 68, 82, 56,
    0, 0, 0, 64, 0, 0, 0, 14, 0, 0, 0, 98, 16, 0, 3, 242, 16, 16, 0, 0, 0, 0, 0, 101, 0, 0, 3, 242,
    32, 16, 0, 0, 0, 0, 0, 54, 0, 0, 5, 242, 32, 16, 0, 0, 0, 0, 0, 70, 30, 16, 0, 0, 0, 0, 0, 62,
    0, 0, 1, 73, 83, 71, 78, 40, 0, 0, 0, 1, 0, 0, 0, 8, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 15, 15, 0, 0, 67, 79, 76, 79, 82, 0, 171, 171, 79, 83, 71, 78, 44,
    0, 0, 0, 1, 0, 0, 0, 8, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    15, 0, 0, 0, 83, 86, 95, 84, 65, 82, 71, 69, 84, 0, 171, 171,
];
