use crate::unity_interface::{Interface, UnityInterfaceGUID};

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum XRLogType {
    /// LogType used for Errors.
    Error = 0,
    /// LogType used for Asserts. (These indicate an error inside Unity itself.)
    Assert = 1,
    /// LogType used for Warnings.
    Warning = 2,
    /// LogType used for regular log messages.
    Log = 3,
    /// LogType used for Exceptions.
    Exception = 4,
    /// LogType used for Debug.
    Debug = 5,
    ///
    NumLevels,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IUnityXRTrace {
    //this is interesting. for the unity headers, on x86 this would use stdcall, which doesn't
    //support variadic functions (because the callee cleans up I think). So in theory it'd crash, right?
    //Nope, according to MS docs the compiler ignores stdcall if the function has varargs and makes it
    //cdecl instead. I wonder what rustc does.
    //It complains and says "use cdecl". Cool.
    pub trace: unsafe extern "C" fn(log_type: XRLogType, *const std::os::raw::c_char, ...),
}

impl Interface for IUnityXRTrace {
    const GUID_HIGH: u64 = 0xC633A7C9398B4A95;
    const GUID_LOW: u64 = 0xC225399ED5A2328F;
    const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}
