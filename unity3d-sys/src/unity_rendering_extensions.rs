//So Rust doesn't like when an enum variant has the same value as another:
//One solution is to use associated constants instead:
//https://stackoverflow.com/a/38025554
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum UnityRenderingExtEventType {
    // issued during SetStereoTarget and carrying the current 'eye' index as parameter
    SetStereoTarget,
    // issued during stereo rendering at the beginning of each eye's rendering loop. It carries the current 'eye' index as parameter
    SetStereoEye,
    // issued after the rendering has finished
    StereoRenderingDone,
    // issued during BeforeDrawCall and carrying UnityRenderingExtBeforeDrawCallParams as parameter
    BeforeDrawCall,
    // issued during AfterDrawCall. This event doesn't carry any parameters
    AfterDrawCall,
    // issued during GrabIntoRenderTexture since we can't simply copy the resources
    // when custom rendering is used - we need to let plugin handle this. It carries over
    // a UnityRenderingExtCustomBlitParams params = { X, source, dest, 0, 0 } ( X means it's irrelevant )
    CustomGrab,
    // issued by plugin to insert custom blits. It carries over UnityRenderingExtCustomBlitParams as param.
    CustomBlit,
    // Deprecated.
    UpdateTextureBegin,
    // Deprecated.
    UpdateTextureEnd,
    // Deprecated. Issued to update a texture. It carries over UnityRenderingExtTextureUpdateParamsV1
    // UpdateTextureBeginV1 = UpdateTextureBegin,
    // Deprecated. Issued to signal the plugin that the texture update has finished. It carries over the same UnityRenderingExtTextureUpdateParamsV1 as UpdateTextureBeginV1
    // UpdateTextureEndV1 = UpdateTextureEnd,
    // Issued to update a texture. It carries over UnityRenderingExtTextureUpdateParamsV2
    UpdateTextureBeginV2,
    // Issued to signal the plugin that the texture update has finished. It carries over the same UnityRenderingExtTextureUpdateParamsV2 as UpdateTextureBeginV2
    UpdateTextureEndV2,

    // keep this last
    Count,
    // kUnityRenderingExtUserEventsStart = Count,
}

//rename this later maybe to fit convention. Or not.
#[allow(non_upper_case_globals)]
impl UnityRenderingExtEventType {
    pub const UpdateTextureBeginV1: UnityRenderingExtEventType =
        UnityRenderingExtEventType::UpdateTextureBegin;
    pub const UpdateTextureEndV1: UnityRenderingExtEventType =
        UnityRenderingExtEventType::UpdateTextureEnd;
    pub const UserEventsStart: UnityRenderingExtEventType = UnityRenderingExtEventType::Count;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtCustomBlitCommands {
    VRFlush, // This event is mostly used in multi GPU configurations ( SLI, etc ) in order to allow the plugin to flush all GPU's targets

    // keep this last
    Count,
    // kUnityRenderingExtUserCustomBlitStart = Count,
}

#[allow(non_upper_case_globals)]
impl UnityRenderingExtCustomBlitCommands {
    pub const UserCustomBlitStart: UnityRenderingExtCustomBlitCommands =
        UnityRenderingExtCustomBlitCommands::Count;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtQueryType {
    OverrideViewport = 1 << 0, // The plugin handles setting up the viewport rects. Unity will skip its internal SetViewport calls
    OverrideScissor = 1 << 1, // The plugin handles setting up the scissor rects. Unity will skip its internal SetScissor calls
    OverrideVROcclussionMesh = 1 << 2, // The plugin handles its own VR occlusion mesh rendering. Unity will skip rendering its internal VR occlusion mask
    OverrideVRSinglePass = 1 << 3, // The plugin uses its own single pass stereo technique. Unity will only traverse and render the render node graph once.
    //      and it will clear the whole render target not just per-eye on demand.
    KeepOriginalDoubleWideWidth_DEPRECATED = 1 << 4, // Instructs unity to keep the original double wide width. By default unity will try and have a power-of-two width for mip-mapping requirements.
    RequestVRFlushCallback = 1 << 5, // Instructs unity to provide callbacks when the VR eye textures need flushing. Useful for multi GPU synchronization.
}
