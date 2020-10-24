use crate::unity_interface::{Interface, UnityInterfaceGUID};

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum UnityGfxRenderer {
    //OpenGL            =  0, // Legacy OpenGL, removed
    //D3D9              =  1, // Direct3D 9, removed
    D3D11 = 2,       // Direct3D 11
    Null = 4,        // "null" device (used in batch mode)
    OpenGLES20 = 8,  // OpenGL ES 2.0
    OpenGLES30 = 11, // OpenGL ES 3.0
    //GXM               = 12, // PlayStation Vita, removed
    PS4 = 13,          // PlayStation 4
    XboxOne = 14,      // Xbox One
    Metal = 16,        // iOS Metal
    OpenGLCore = 17,   // OpenGL core
    D3D12 = 18,        // Direct3D 12
    Vulkan = 21,       // Vulkan
    Nvn = 22,          // Nintendo Switch NVN API
    XboxOneD3D12 = 23, // MS XboxOne Direct3D 12
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum UnityGfxDeviceEventType {
    Initialize = 0,
    Shutdown = 1,
    BeforeReset = 2,
    AfterReset = 3,
}

type IUnityGraphicsDeviceEventCallback =
    unsafe extern "system" fn(event_type: UnityGfxDeviceEventType);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnityGraphics {
    pub get_renderer: unsafe extern "system" fn() -> UnityGfxRenderer,
    pub register_device_event_callback:
        unsafe extern "system" fn(callback: IUnityGraphicsDeviceEventCallback),
    pub unregister_device_event_callback:
        unsafe extern "system" fn(callback: IUnityGraphicsDeviceEventCallback),
    pub reserve_event_id_range: unsafe extern "system" fn(count: i32) -> i32,
}

impl Interface for IUnityGraphics {
    //this is seriously cool. It looks so nice at the call site and works for both split and regular fns :)
    const GUID_HIGH: u64 = 0x7CBA0A9CA4DDB544;
    const GUID_LOW: u64 = 0x8C5AD4926EB17B11;
    const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}

pub type UnityRenderingEvent = unsafe extern "system" fn(event_id: i32);
pub type UnityRenderingEventAndData =
    unsafe extern "system" fn(event_id: i32, data: *const std::os::raw::c_void);
