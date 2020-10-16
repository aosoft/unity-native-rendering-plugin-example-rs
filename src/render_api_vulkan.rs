pub fn on_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    unsafe {
        interfaces
            .interface::<unity_native_plugin_vulkan::vulkan::UnityGraphicsVulkan>()
            .unwrap()
            .intercept_initialization(
                Some(crate::vulkan_api::vulkan_functions::intercept_vulkan_initialization),
                std::ptr::null_mut(),
            );
    }
}
