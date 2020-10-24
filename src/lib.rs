use unity3d_sys::unity_interface::{IUnityInterface, IUnityInterfaces, UnityInterfaceGUID};
use unity3d_sys::unity_xr_trace::{IUnityXRTrace, XRLogType};
use unity3d_sys::winapi::um::d3d11::{ID3D11DeviceContext, ID3D11RenderTargetView, D3D11_VIEWPORT};
use unity3d_sys::{
    unity_graphics::{IUnityGraphics, UnityGfxDeviceEventType, UnityRenderingEvent},
    unity_rendering_extensions::UnityRenderingExtEventType,
};
use unity3d_sys::{
    unity_graphics_d3d11::IUnityGraphicsD3D11,
    unity_rendering_extensions::UnityRenderingExtQueryType,
};

//very much unsafe: https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics
//we'll assign to this in UnityPluginLoad and use it later in a rendering call.
static mut D3D11_GFX: Option<IUnityGraphicsD3D11> = None;

#[no_mangle]
pub unsafe extern "system" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
    println!("Heyo, I'm a rust function!");

    let unity = IUnityInterfaces::from_raw(unity_interfaces);
    let graphics = unity.get_interface::<IUnityGraphics>();

    if let Some(graphics) = graphics {
        println!("Got IUnityGraphics!");
        let get_renderer = graphics.get_renderer;
        let renderer = get_renderer();

        let register_device_event_callback = graphics.register_device_event_callback;
        register_device_event_callback(on_graphics_device_event);

        //pity that this doesn't print to debugger console :/
        //but nice, it prints to the editor log, how cool is that! println! does too :)
        eprintln!("Current renderer: {:?}", renderer);
    }

    let xr_trace = unity.get_interface::<IUnityXRTrace>();
    if let Some(xr_trace) = xr_trace {
        println!("Got IUnityXRTrace!");

        let trace_fn = xr_trace.trace;

        let message = std::ffi::CString::new("Yo from XRTrace").expect("something exploded");
        trace_fn(XRLogType::Log, message.as_ptr());
    }

    let d3d_gfx = unity.get_interface::<IUnityGraphicsD3D11>();
    if d3d_gfx.is_some() {
        println!("Got IUnityGraphicsD3D11!");

        //so now we wanna save this so we can later use it from the callback that
        //is called when we do GL.IssuePluginEvent in C#.
        D3D11_GFX = d3d_gfx;
    }

    register_test_interface(unity);
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

//This one gets called for low level rendering events, no matter how your plugin is called.
//The GfxPlugin* prefix is only needed for preloading. More info:
//https://docs.unity3d.com/Manual/LowLevelNativePluginRenderingExtensions.html
#[no_mangle]
pub unsafe extern "system" fn UnityRenderingExtEvent(
    event: UnityRenderingExtEventType,
    _data: *mut std::os::raw::c_void,
) {
    match event {
        UnityRenderingExtEventType::BeforeDrawCall => {
            //transmute data to UnityRenderingExtBeforeDrawCallParams
        }
        UnityRenderingExtEventType::AfterDrawCall => {
            //there doesn't seem to be data but we can run code here.
        }
        _ => {}
    }
    // println!("UnityRenderingExtEvent called with event type {:?}", event);
}

//This one gets called for low level rendering queries.
#[no_mangle]
pub unsafe extern "system" fn UnityRenderingExtQuery(_query: UnityRenderingExtQueryType) {
    //do something!
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

fn register_test_interface(unity: IUnityInterfaces) {
    //I wonder how register_interface works since there's no docs.
    //My guess would be that there is a global instance of a struct and we pass an opaque pointer and a guid
    //to unity. Unity then saves that and if someone asks for the same guid, we get the pointer back. Let's
    //test that.
    let test_guid = UnityInterfaceGUID {
        high: 0xDDB39EF89A89D948,
        low: 0xBF76967F17EFB177,
    };
    let register_fn = unity.register_interface;
    let get_interface_fn = unity.get_interface_fn;
    let num = 42;
    let ptr: *const IUnityInterface = num as *const _; //this is just a hack to test what unity gives us back
    println!("ptr: {:p}", ptr);

    unsafe {
        register_fn(test_guid, ptr);
        let result_ptr = get_interface_fn(test_guid);
        println!("result_ptr: {:p}", result_ptr);
    }

    //Alright, my assumption holds. We get back whatever we pass in, which is cool.
    //I wonder how interface creation would look like in Rust. Also, what about C compat?
    //That's like extra work. Yeah, maybe at some later point if I care.
}
