use crate::unity_interface::UnityInterfaceGUID;

//interesting thing I think I could try: clearing a portion of the viewport only. 
//No need for shaders and can test relatively quickly, right?

//Also, this might be an opportnuity to use cargo features since this is windows only, so depends on winapi crate
//unless there's some other crate that wraps d3d11 for us to use

#[repr(C)]
pub struct IUnityGraphicsD3D11 {
    get_device: unsafe extern "system" fn() -> *mut std::os::raw::c_void,
    texture_from_render_buffer: unsafe extern "system" fn() -> *mut std::os::raw::c_void,
    texture_from_native_texture: unsafe extern "system" fn() -> *mut std::os::raw::c_void,
    rtv_from_render_buffer: unsafe extern "system" fn() -> *mut std::os::raw::c_void,
    srv_from_native_texture: unsafe extern "system" fn() -> *mut std::os::raw::c_void,
}

impl IUnityGraphicsD3D11 {
    pub const GUID_HIGH: u64 = 0xAAB37EF87A87D748;
    pub const GUID_LOW: u64 = 0xBF76967F07EFB177;
    pub const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}