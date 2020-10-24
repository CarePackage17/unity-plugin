//So Rust doesn't like when an enum variant has the same value as another:
//One solution is to use associated constants instead:
//https://stackoverflow.com/a/38025554
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtEventType {
    kUnityRenderingExtEventSetStereoTarget, // issued during SetStereoTarget and carrying the current 'eye' index as parameter
    kUnityRenderingExtEventSetStereoEye, // issued during stereo rendering at the beginning of each eye's rendering loop. It carries the current 'eye' index as parameter
    kUnityRenderingExtEventStereoRenderingDone, // issued after the rendering has finished
    kUnityRenderingExtEventBeforeDrawCall, // issued during BeforeDrawCall and carrying UnityRenderingExtBeforeDrawCallParams as parameter
    kUnityRenderingExtEventAfterDrawCall, // issued during AfterDrawCall. This event doesn't carry any parameters
    kUnityRenderingExtEventCustomGrab, // issued during GrabIntoRenderTexture since we can't simply copy the resources
    //      when custom rendering is used - we need to let plugin handle this. It carries over
    //      a UnityRenderingExtCustomBlitParams params = { X, source, dest, 0, 0 } ( X means it's irrelevant )
    kUnityRenderingExtEventCustomBlit, // issued by plugin to insert custom blits. It carries over UnityRenderingExtCustomBlitParams as param.
    kUnityRenderingExtEventUpdateTextureBegin, // Deprecated.
    kUnityRenderingExtEventUpdateTextureEnd, // Deprecated.
    // kUnityRenderingExtEventUpdateTextureBeginV1 = kUnityRenderingExtEventUpdateTextureBegin, // Deprecated. Issued to update a texture. It carries over UnityRenderingExtTextureUpdateParamsV1
    // kUnityRenderingExtEventUpdateTextureEndV1 = kUnityRenderingExtEventUpdateTextureEnd, // Deprecated. Issued to signal the plugin that the texture update has finished. It carries over the same UnityRenderingExtTextureUpdateParamsV1 as kUnityRenderingExtEventUpdateTextureBeginV1
    kUnityRenderingExtEventUpdateTextureBeginV2, // Issued to update a texture. It carries over UnityRenderingExtTextureUpdateParamsV2
    kUnityRenderingExtEventUpdateTextureEndV2, // Issued to signal the plugin that the texture update has finished. It carries over the same UnityRenderingExtTextureUpdateParamsV2 as kUnityRenderingExtEventUpdateTextureBeginV2

    // keep this last
    kUnityRenderingExtEventCount,
    // kUnityRenderingExtUserEventsStart = kUnityRenderingExtEventCount,
}

//rename this later maybe to fit convention. Or not.
#[allow(non_upper_case_globals)]
impl UnityRenderingExtEventType {
    pub const kUnityRenderingExtEventUpdateTextureBeginV1: UnityRenderingExtEventType =
        UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureBegin;
    pub const kUnityRenderingExtEventUpdateTextureEndV1: UnityRenderingExtEventType =
        UnityRenderingExtEventType::kUnityRenderingExtEventUpdateTextureEnd;
    pub const kUnityRenderingExtUserEventsStart: UnityRenderingExtEventType =
        UnityRenderingExtEventType::kUnityRenderingExtEventCount;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtCustomBlitCommands {
    kUnityRenderingExtCustomBlitVRFlush, // This event is mostly used in multi GPU configurations ( SLI, etc ) in order to allow the plugin to flush all GPU's targets

    // keep this last
    kUnityRenderingExtCustomBlitCount,
    // kUnityRenderingExtUserCustomBlitStart = kUnityRenderingExtCustomBlitCount,
}

#[allow(non_upper_case_globals)]
impl UnityRenderingExtCustomBlitCommands {
    pub const kUnityRenderingExtUserCustomBlitStart: UnityRenderingExtCustomBlitCommands =
        UnityRenderingExtCustomBlitCommands::kUnityRenderingExtCustomBlitCount;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtQueryType
{
    kUnityRenderingExtQueryOverrideViewport             = 1 << 0,           // The plugin handles setting up the viewport rects. Unity will skip its internal SetViewport calls
    kUnityRenderingExtQueryOverrideScissor              = 1 << 1,           // The plugin handles setting up the scissor rects. Unity will skip its internal SetScissor calls
    kUnityRenderingExtQueryOverrideVROcclussionMesh     = 1 << 2,           // The plugin handles its own VR occlusion mesh rendering. Unity will skip rendering its internal VR occlusion mask
    kUnityRenderingExtQueryOverrideVRSinglePass         = 1 << 3,           // The plugin uses its own single pass stereo technique. Unity will only traverse and render the render node graph once.
                                                                            //      and it will clear the whole render target not just per-eye on demand.
    kUnityRenderingExtQueryKeepOriginalDoubleWideWidth_DEPRECATED  = 1 << 4,           // Instructs unity to keep the original double wide width. By default unity will try and have a power-of-two width for mip-mapping requirements.
    kUnityRenderingExtQueryRequestVRFlushCallback       = 1 << 5,           // Instructs unity to provide callbacks when the VR eye textures need flushing. Useful for multi GPU synchronization.
}
