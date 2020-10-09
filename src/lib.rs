// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub mod unity_graphics;
pub mod unity_interface;
pub mod unity_xr_trace;

use crate::unity_graphics::{IUnityGraphics, UnityGfxDeviceEventType};
use crate::unity_interface::IUnityInterfaces;
use crate::unity_xr_trace::{IUnityXRTrace, XRLogType};

#[no_mangle]
pub unsafe extern "system" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
    println!("Heyo, I'm a rust function!");

    //https://stackoverflow.com/a/27994682
    let get_iface = (*unity_interfaces).get_interface;
    let gfx_interface_ptr = get_iface(IUnityGraphics::GUID);

    if gfx_interface_ptr != std::ptr::null() {
        println!("Got IUnityGraphics!");
        let gfx_interface = &*(gfx_interface_ptr as *const IUnityGraphics);
        let get_renderer = gfx_interface.get_renderer;
        let renderer = get_renderer();

        let register_device_event_callback = gfx_interface.register_device_event_callback;
        register_device_event_callback(on_graphics_device_event);

        //pity that this doesn't print to debugger console :/
        //but nice, it prints to the editor log, how cool is that! println! does too :)
        eprintln!("Current renderer: {:?}", renderer);
    }

    let xr_trace_ptr = get_iface(IUnityXRTrace::GUID);
    if xr_trace_ptr != std::ptr::null() {
        println!("Got IUnityXRTrace!");
        //I think we could safely make this Copy since it's just a bunch of fn pointers, right?
        let xr_trace = &*(xr_trace_ptr as *const IUnityXRTrace);
        let trace = xr_trace.trace;

        let message = std::ffi::CString::new("Yo from XRTrace").expect("something exploded");
        trace(XRLogType::kXRLogTypeLog, message.as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "system" fn UnityPluginUnload() {
    println!("I'm never gonna be called from the editor!");
}

unsafe extern "system" fn on_graphics_device_event(event_type: UnityGfxDeviceEventType) {
    //interestingly this gets called multiple times when we exit the editor. but it works :)
    println!("We got a graphics device event: {:?}", event_type);
}
