use crate::unity_interface::UnityInterfaceGUID;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum XRLogType {
    /// LogType used for Errors.
    kXRLogTypeError = 0,
    /// LogType used for Asserts. (These indicate an error inside Unity itself.)
    kXRLogTypeAssert = 1,
    /// LogType used for Warnings.
    kXRLogTypeWarning = 2,
    /// LogType used for regular log messages.
    kXRLogTypeLog = 3,
    /// LogType used for Exceptions.
    kXRLogTypeException = 4,
    /// LogType used for Debug.
    kXRLogTypeDebug = 5,
    ///
    kXRLogTypeNumLevels,
}

pub struct IUnityXRTrace {
    //this is interesting. for the unity headers, on x86 this would use stdcall, which doesn't 
    //support variadic functions (because the callee cleans up I think). So in theory it'd crash, right?
    pub trace: unsafe extern "C" fn(log_type: XRLogType, *const std::os::raw::c_char, ...),
}

impl IUnityXRTrace {
    pub const GUID_HIGH: u64 = 0xC633A7C9398B4A95;
    pub const GUID_LOW: u64 = 0xC225399ED5A2328F;
    pub const GUID: UnityInterfaceGUID = UnityInterfaceGUID {
        high: Self::GUID_HIGH,
        low: Self::GUID_LOW,
    };
}