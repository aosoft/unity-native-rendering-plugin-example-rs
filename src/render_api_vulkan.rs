extern "system" fn intercept_vulkan_initialization(
    get_instance_proc_addr: unity_native_plugin_vulkan::vulkan::PFN_vkGetInstanceProcAddr,
    userdata: *mut ::std::os::raw::c_void,
) -> unity_native_plugin_vulkan::vulkan::PFN_vkGetInstanceProcAddr {
    None
}

pub fn on_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    unsafe {
        interfaces
            .interface::<unity_native_plugin_vulkan::vulkan::UnityGraphicsVulkan>()
            .unwrap()
            .intercept_initialization(Some(intercept_vulkan_initialization), std::ptr::null_mut());
    }
}
