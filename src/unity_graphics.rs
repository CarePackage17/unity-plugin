use crate::unity_interface::UnityInterfaceGUID;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum UnityGfxRenderer {
    //kUnityGfxRendererOpenGL            =  0, // Legacy OpenGL, removed
    //kUnityGfxRendererD3D9              =  1, // Direct3D 9, removed
    kUnityGfxRendererD3D11 = 2,       // Direct3D 11
    kUnityGfxRendererNull = 4,        // "null" device (used in batch mode)
    kUnityGfxRendererOpenGLES20 = 8,  // OpenGL ES 2.0
    kUnityGfxRendererOpenGLES30 = 11, // OpenGL ES 3.0
    //kUnityGfxRendererGXM               = 12, // PlayStation Vita, removed
    kUnityGfxRendererPS4 = 13,          // PlayStation 4
    kUnityGfxRendererXboxOne = 14,      // Xbox One
    kUnityGfxRendererMetal = 16,        // iOS Metal
    kUnityGfxRendererOpenGLCore = 17,   // OpenGL core
    kUnityGfxRendererD3D12 = 18,        // Direct3D 12
    kUnityGfxRendererVulkan = 21,       // Vulkan
    kUnityGfxRendererNvn = 22,          // Nintendo Switch NVN API
    kUnityGfxRendererXboxOneD3D12 = 23, // MS XboxOne Direct3D 12
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum UnityGfxDeviceEventType {
    kUnityGfxDeviceEventInitialize = 0,
    kUnityGfxDeviceEventShutdown = 1,
    kUnityGfxDeviceEventBeforeReset = 2,
    kUnityGfxDeviceEventAfterReset = 3,
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

impl IUnityGraphics {
    //this is seriously cool. It looks so nice at the call site and works for both split and regular fns :)
    pub const GUID_HIGH: u64 = 0x7CBA0A9CA4DDB544;
    pub const GUID_LOW: u64 = 0x8C5AD4926EB17B11;
    pub const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}

pub type UnityRenderingEvent = unsafe extern "system" fn(event_id: i32);
pub type UnityRenderingEventAndData =
    unsafe extern "system" fn(event_id: i32, data: *const std::os::raw::c_void);
