use unity3d_sys::{
    unity_graphics::IUnityGraphics,
    unity_graphics_d3d11::IUnityGraphicsD3D11,
    unity_interface::{IUnityInterfaces, UnityInterfaceGUID},
    unity_xr_trace::IUnityXRTrace,
};

//Also, I think we can assume it's just going to be bags of function pointers, right?
//So let's make it a subtrait of copy.
//Not sure if we can assume that. A Unity interface can be just about anything we pass to
//register_interface (it's just a pointer). I think it's safe to assume that all builtin
//ones are just function pointers, but theoretically they could be any data.
pub trait Interface: Copy {
    const GUID_LOW: u64;
    const GUID_HIGH: u64;
    const GUID: UnityInterfaceGUID;
}

impl Interface for IUnityGraphicsD3D11 {
    const GUID_HIGH: u64 = unity3d_sys::unity_graphics_d3d11::GUID_HIGH;
    const GUID_LOW: u64 = unity3d_sys::unity_graphics_d3d11::GUID_LOW;
    const GUID: UnityInterfaceGUID = unity3d_sys::unity_graphics_d3d11::GUID;
}

impl Interface for IUnityGraphics {
    const GUID_HIGH: u64 = unity3d_sys::unity_graphics::GUID_HIGH;
    const GUID_LOW: u64 = unity3d_sys::unity_graphics::GUID_LOW;
    const GUID: UnityInterfaceGUID = unity3d_sys::unity_graphics::GUID;
}

impl Interface for IUnityXRTrace {
    const GUID_HIGH: u64 = unity3d_sys::unity_xr_trace::GUID_HIGH;
    const GUID_LOW: u64 = unity3d_sys::unity_xr_trace::GUID_LOW;
    const GUID: UnityInterfaceGUID = unity3d_sys::unity_xr_trace::GUID;
}

//higher-level wrapper for IUnityInterfaces
pub struct UnityInterfaceRegistry {
    //right now this is public. Consider making it private when we wrapped register fn.
    pub raw: IUnityInterfaces,
}

impl UnityInterfaceRegistry {
    pub fn get_interface<T: Interface>(&self) -> Option<T> {
        //About calling function pointers from a struct:
        //https://stackoverflow.com/a/27994682
        let interface_ptr = unsafe { (self.raw.get_interface_fn)(T::GUID) };
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

    //how would a safe register_interface look like? It kinda needs to take care of lifetimes, doesn't it?
    //otherwise we could take a pointer to a temporary, register that but then have a dangling pointer
    //after the temp variable is gone. I wonder if there's a way to prevent that.
    //Maybe a wrapper that calls unregister? Wait, there is no unregister...maybe require static lifetime?
    //I wonder how other people do this kind of "userdata" pattern. I'm sure there's some code out there
    //that already solved this.

    //I'm not sure if we can ever verify safety since somebody can always pass a
    //garbage pointer...so let's leave it unsafe for now.
    pub unsafe fn from_raw(ptr: *const IUnityInterfaces) -> Self {
        Self {
            raw: IUnityInterfaces {
                get_interface_fn: (*ptr).get_interface_fn,
                register_interface: (*ptr).register_interface,
                get_interface_split_fn: (*ptr).get_interface_split_fn,
                register_interface_split: (*ptr).register_interface_split,
            },
        }
    }
}
