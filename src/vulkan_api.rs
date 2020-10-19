use unity_native_plugin::graphics::GfxRenderer::Vulkan;

pub struct vulkan_functions {
    static_fn: ash::vk::StaticFn,
    entry_fn_1_0: ash::vk::EntryFnV1_0,
    entry_fn_1_1: ash::vk::EntryFnV1_1,
    entry_fn_1_2: ash::vk::EntryFnV1_2,
}

static mut VULKAN_FUNCTIONS: Option<vulkan_functions> = None;

impl vulkan_functions {
    pub unsafe fn default() -> &'static vulkan_functions {
        vulkan_functions::default_mut()
    }

    unsafe fn default_mut() -> &'static mut vulkan_functions {
        if VULKAN_FUNCTIONS.is_none() {
            VULKAN_FUNCTIONS = Some(std::mem::zeroed());
        }
        VULKAN_FUNCTIONS.as_mut().unwrap()
    }

    fn new(get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr) -> vulkan_functions {
        let static_fn = ash::vk::StaticFn {
            get_instance_proc_addr,
        };

        let entry_fn_1_0 = ash::vk::EntryFnV1_0::load(|name| unsafe {
            std::mem::transmute(
                static_fn.get_instance_proc_addr(ash::vk::Instance::null(), name.as_ptr()),
            )
        });

        let entry_fn_1_1 = ash::vk::EntryFnV1_1::load(|name| unsafe {
            std::mem::transmute(
                static_fn.get_instance_proc_addr(ash::vk::Instance::null(), name.as_ptr()),
            )
        });

        let entry_fn_1_2 = ash::vk::EntryFnV1_2::load(|name| unsafe {
            std::mem::transmute(
                static_fn.get_instance_proc_addr(ash::vk::Instance::null(), name.as_ptr()),
            )
        });

        vulkan_functions {
            static_fn,
            entry_fn_1_0,
            entry_fn_1_1,
            entry_fn_1_2,
        }
    }

    pub extern "system" fn intercept_vulkan_initialization(
        get_instance_proc_addr: ash::vk::PFN_vkGetInstanceProcAddr,
        _: *mut ::std::os::raw::c_void,
    ) -> ash::vk::PFN_vkGetInstanceProcAddr {
        unsafe {
            VULKAN_FUNCTIONS = Some(vulkan_functions::new(get_instance_proc_addr));
            vulkan_functions::hook_vk_get_instance_proc_addr
        }
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
                    Some(std::mem::transmute(vulkan_functions::hook_vk_create_instance as usize))
                } else {
                    None
                }
            }
        }
    }

    unsafe extern "system" fn hook_vk_create_instance(
        p_create_info: *const ash::vk::InstanceCreateInfo,
        p_allocator: *const ash::vk::AllocationCallbacks,
        p_instance: *mut ash::vk::Instance,
    ) -> ash::vk::Result {
        Self::default().entry_fn_1_0.create_instance(p_create_info, p_allocator, p_instance)
    }
}
