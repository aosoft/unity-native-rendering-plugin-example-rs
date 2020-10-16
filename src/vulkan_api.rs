use unity_native_plugin::graphics::GfxRenderer::Vulkan;

pub struct vulkan_functions {
    get_instance_proc_addr: Option<ash::vk::PFN_vkGetInstanceProcAddr>,
}

static mut VULKAN_FUNCTIONS: Option<vulkan_functions> = None;

impl vulkan_functions {
    pub fn default() -> &'static vulkan_functions {
        vulkan_functions::default_mut()
    }

    fn default_mut() -> &'static mut vulkan_functions {
        unsafe {
            if VULKAN_FUNCTIONS.is_none() {
                VULKAN_FUNCTIONS = Some(std::mem::zeroed());
            }
            VULKAN_FUNCTIONS.as_mut().unwrap()
        }
    }

    pub extern "system" fn intercept_vulkan_initialization(
        get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr,
        _: *mut ::std::os::raw::c_void,
    ) -> ash::vk::PFN_vkGetInstanceProcAddr {
        vulkan_functions::default_mut().get_instance_proc_addr = Some(get_instance_proc_addr);
        vulkan_functions::hook_vk_get_instance_proc_addr
    }

    extern "system" fn hook_vk_get_instance_proc_addr(
        _: ash::vk::Instance,
        func_name: *const std::os::raw::c_char,
    ) -> ash::vk::PFN_vkVoidFunction {
        if func_name.is_null() {
            None
        } else {
            unsafe {
                let func_name = std::ffi::CStr::from_ptr(func_name);
                if func_name.to_bytes() == "vkCreateInstance".as_bytes() {
                    Some(vulkan_functions::hook_vk_create_instance)
                } else {
                    None
                }
            }
        }
    }

    extern "system" fn hook_vk_create_instance() -> std::ffi::c_void {
        unsafe { std::mem::zeroed::<std::ffi::c_void>() }
    }
}
