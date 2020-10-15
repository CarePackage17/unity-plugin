use crate::unity_interface::{UnityInterfaceGUID, UnityRenderBuffer, UnityTextureID};
use winapi::um::d3d11::ID3D11Device;

//interesting thing I think I could try: clearing a portion of the viewport only. 
//No need for shaders and can test relatively quickly, right?

//Also, this might be an opportunity to use cargo features since this is windows only.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnityGraphicsD3D11 {
    pub get_device: unsafe extern "system" fn() -> *const ID3D11Device,
    pub texture_from_render_buffer: unsafe extern "system" fn(buffer: UnityRenderBuffer) -> *const std::os::raw::c_void,
    pub texture_from_native_texture: unsafe extern "system" fn(id: UnityTextureID) -> *const std::os::raw::c_void,
    pub rtv_from_render_buffer: unsafe extern "system" fn(buffer: UnityRenderBuffer) -> *const std::os::raw::c_void,
    pub srv_from_native_texture: unsafe extern "system" fn(id: UnityTextureID) -> *const std::os::raw::c_void,
}

impl IUnityGraphicsD3D11 {
    pub const GUID_HIGH: u64 = 0xAAB37EF87A87D748;
    pub const GUID_LOW: u64 = 0xBF76967F07EFB177;
    pub const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}