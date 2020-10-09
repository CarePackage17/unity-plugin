#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod unity_interface;
pub mod unity_graphics;

use crate::unity_interface::IUnityInterfaces;
use crate::unity_graphics::IUnityGraphics;

#[no_mangle]
pub unsafe extern "system" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
    println!("Heyo, I'm a rust function!");

    //https://stackoverflow.com/a/27994682
    let get_iface = (*unity_interfaces).get_interface;
    let gfx_interface = get_iface(IUnityGraphics::GUID);

    if gfx_interface != std::ptr::null() {
        println!("Got IUnityGraphics!");

        let get_renderer = (*(gfx_interface as *const IUnityGraphics)).get_renderer;
        let renderer = get_renderer();

        //pity that this doesn't print to debugger console :/
        eprintln!("Curernt renderer: {:?}", renderer);
    }
}

#[no_mangle]
pub unsafe extern "system" fn UnityPluginUnload() {
    println!("I'm never gonna be called from the editor!");
}
