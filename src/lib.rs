use unity3d_sys::unity_graphics::{IUnityGraphics, UnityGfxDeviceEventType, UnityRenderingEvent};
use unity3d_sys::unity_graphics_d3d11::IUnityGraphicsD3D11;
use unity3d_sys::unity_interface::IUnityInterfaces;
use unity3d_sys::unity_xr_trace::{IUnityXRTrace, XRLogType};
use unity3d_sys::winapi::um::d3d11::{ID3D11DeviceContext, ID3D11RenderTargetView, D3D11_VIEWPORT};

//very much unsafe: https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics
//we'll assign to this in UnityPluginLoad and use it later in a rendering call.
static mut D3D11_GFX: Option<IUnityGraphicsD3D11> = None;

#[no_mangle]
pub unsafe extern "system" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
    println!("Heyo, I'm a rust function!");

    //https://stackoverflow.com/a/27994682
    let get_iface = (*unity_interfaces).get_interface;
    let gfx_interface_ptr = get_iface(IUnityGraphics::GUID);

    if gfx_interface_ptr != std::ptr::null() {
        println!("Got IUnityGraphics!");
        let gfx_interface = *(gfx_interface_ptr as *const IUnityGraphics);
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
        let xr_trace = *(xr_trace_ptr as *const IUnityXRTrace);
        let trace = xr_trace.trace;

        let message = std::ffi::CString::new("Yo from XRTrace").expect("something exploded");
        trace(XRLogType::kXRLogTypeLog, message.as_ptr());
    }

    let d3d_gfx_ptr = get_iface(IUnityGraphicsD3D11::GUID);
    if d3d_gfx_ptr != std::ptr::null() {
        println!("Got IUnityGraphicsD3D11!");
        let d3d_gfx = *(d3d_gfx_ptr as *const IUnityGraphicsD3D11);

        //so now we wanna save this so we can later use it from the callback that
        //is called when we do GL.IssuePluginEvent in C#.
        D3D11_GFX = Some(d3d_gfx);
    }
}

#[no_mangle]
pub unsafe extern "system" fn UnityPluginUnload() {
    println!("I'm never gonna be called from the editor!");
}

//this can be used from unity via DllImport and then calling GL.IssuePluginEvent with this pointer
#[no_mangle]
pub unsafe extern "system" fn DoGraphicsStuff() -> UnityRenderingEvent {
    do_graphics_stuff
}

unsafe extern "system" fn do_graphics_stuff(_event_id: i32) {
    //We wanted this to clear only a portion of the view, but ClearRenderTargetView overwrites the whole thing.
    //There is ID3D11DeviceContext1::ClearView which takes a rect, so we could try using that.

    if let Some(d3d11) = D3D11_GFX {
        let get_device = d3d11.get_device;

        let d3d11_device = get_device();
        let mut d3d11_context: *mut ID3D11DeviceContext = std::ptr::null_mut();
        // this needs *mut *mut ID3D11DeviceContext and I'm tired rn.
        d3d11_device
            .as_ref()
            .unwrap()
            .GetImmediateContext(&mut d3d11_context);

        //apparently we can't call methods on raw pointers which I don't understand right now, but the compiler
        //suggested to do this. Let's try it like that for now. Before it was:
        // d3d11_context
        //     .as_ref()
        //     .unwrap()
        //     .RSGetViewports(&mut num_viewports, &mut old_viewport);
        let d3d11_context = d3d11_context.as_ref().unwrap();

        // save old viewport
        let mut old_viewport: D3D11_VIEWPORT = std::mem::zeroed();
        let mut num_viewports: u32 = 1;
        d3d11_context.RSGetViewports(&mut num_viewports, &mut old_viewport);

        //set new viewport. Let's make it a 100px square.
        let new_viewport = D3D11_VIEWPORT {
            TopLeftX: 0f32,
            TopLeftY: 0f32,
            MinDepth: 0f32,
            MaxDepth: 1f32,
            Width: 100f32,
            Height: 100f32,
        };
        d3d11_context.RSSetViewports(1, &new_viewport);

        //clear (we need a ptr to a rendertargetview for that...how do we get one?)
        //so there's rtv_from_render_buffer api, but how do we get the renderbuffer number?
        //there is a render buffer scripting api: https://docs.unity3d.com/ScriptReference/RenderBuffer.GetNativeRenderBufferPtr.html
        //it says it's iOS only, but I don't believe them.
        //Maybe there's a d3d11 api to get the currently bound render target (view)?
        //this seems to be it: https://docs.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11devicecontext-omgetrendertargets

        let mut render_target_view_ptr: *mut ID3D11RenderTargetView = std::ptr::null_mut();
        let color = [0.5, 0.0, 0.0, 1.0f32];
        d3d11_context.OMGetRenderTargets(1, &mut render_target_view_ptr, std::ptr::null_mut());
        d3d11_context.ClearRenderTargetView(render_target_view_ptr, &color);

        //restore old viewport
        d3d11_context.RSSetViewports(1, &old_viewport);

        //NOTE: remember to call release on the rtv, otherwise there will be leaks since OMGetRenderTargets bumps the refcount.
        (*render_target_view_ptr).Release();
    }
}

unsafe extern "system" fn on_graphics_device_event(event_type: UnityGfxDeviceEventType) {
    //interestingly this gets called multiple times when we exit the editor. but it works :)
    println!("We got a graphics device event: {:?}", event_type);
}
