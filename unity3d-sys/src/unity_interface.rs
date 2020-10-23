pub struct UnityInterfaceGUID {
    pub high: u64,
    pub low: u64,
}

// In C this is typedef to void.
#[derive(Copy, Clone)]
pub struct IUnityInterface;

//Alright, let's use this as a marker trait to limit the amount of types that
//can be returned from get_interface. Or actually, we could let this trait hold the
//interface guid. Neat.
//Also, I think we can assume it's just going to be bags of function pointers, right?
//So let's make it a subtrait of copy.
pub trait Interface: Copy {
    const GUID_LOW: u64;
    const GUID_HIGH: u64;
    const GUID: UnityInterfaceGUID;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnityInterfaces {
    // Returns an interface matching the guid.
    // Returns nullptr if the given interface is unavailable in the active Unity runtime.
    pub get_interface_fn:
        unsafe extern "system" fn(guid: UnityInterfaceGUID) -> *const IUnityInterface,
    // Registers a new interface.
    pub register_interface:
        unsafe extern "system" fn(guid: UnityInterfaceGUID, *const IUnityInterface),

    // Split APIs for C
    pub get_interface_split_fn:
        unsafe extern "system" fn(guid_high: u64, guid_low: u64) -> *const IUnityInterface,
    pub register_interface_split:
        unsafe extern "system" fn(guid_high: u64, guid_low: u64, *const IUnityInterface),
}

impl IUnityInterfaces {
    // this should be safe because Unity says if the interface doesn't exist, it'll return nullptr.
    // Otherwise it'll be a valid opaque pointer.
    pub fn get_interface<T: Interface>(&self) -> Option<T> {
        let interface_ptr = unsafe { (self.get_interface_fn)(T::GUID) };
        if interface_ptr == std::ptr::null() {
            None
        } else {
            //cast the pointer to T (assume Unity gives us a valid pointer if it's non-null)
            //and return that.
            //I wonder how safe that is.
            let interface = unsafe { *(interface_ptr as *const T) };
            Some(interface)
        }
    }

    //I'm not sure if we can ever verify safety since somebody can always pass a
    //garbage pointer...so let's leave it unsafe for now.
    pub unsafe fn from_raw(ptr: *const IUnityInterfaces) -> IUnityInterfaces {
        IUnityInterfaces {
            get_interface_fn: (*ptr).get_interface_fn,
            register_interface: (*ptr).register_interface,
            get_interface_split_fn: (*ptr).get_interface_split_fn,
            register_interface_split: (*ptr).register_interface_split,
        }
    }
}

pub struct RenderSurfaceBase;
pub type UnityRenderBuffer = *const RenderSurfaceBase;
pub type UnityTextureID = u32;
