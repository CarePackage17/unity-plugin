pub struct UnityInterfaceGUID {
    pub high: u64,
    pub low: u64,
}

// In C this is typedef to void.
#[derive(Copy, Clone)]
pub struct IUnityInterface;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnityInterfaces {
    // Returns an interface matching the guid.
    // Returns nullptr if the given interface is unavailable in the active Unity runtime.
    pub get_interface:
        unsafe extern "system" fn(guid: UnityInterfaceGUID) -> *const IUnityInterface,
    // Registers a new interface.
    pub register_interface:
        unsafe extern "system" fn(guid: UnityInterfaceGUID, *mut IUnityInterface),

    // Split APIs for C
    pub get_interface_split:
        unsafe extern "system" fn(guid_high: u64, guid_low: u64) -> *const IUnityInterface,
    pub register_interface_split:
        unsafe extern "system" fn(guid_high: u64, guid_low: u64, *mut IUnityInterface),
}

pub struct RenderSurfaceBase;
pub type UnityRenderBuffer = *const RenderSurfaceBase;
pub type UnityTextureID = u32;