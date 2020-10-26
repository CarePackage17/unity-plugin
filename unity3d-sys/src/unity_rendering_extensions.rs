use std::ffi::c_void;

use crate::unity_interface::{UnityRenderBuffer, UnityTextureID};

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

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum UnityRenderingExtTextureFormat {
    None = 0, //kUnityRenderingExtFormatFirst = kUnityRenderingExtFormatNone,

    // sRGB formats
    R8_SRGB,
    R8G8_SRGB,
    R8G8B8_SRGB,
    R8G8B8A8_SRGB,

    // 8 bit integer formats
    R8_UNorm,
    R8G8_UNorm,
    R8G8B8_UNorm,
    R8G8B8A8_UNorm,
    R8_SNorm,
    R8G8_SNorm,
    R8G8B8_SNorm,
    R8G8B8A8_SNorm,
    R8_UInt,
    R8G8_UInt,
    R8G8B8_UInt,
    R8G8B8A8_UInt,
    R8_SInt,
    R8G8_SInt,
    R8G8B8_SInt,
    R8G8B8A8_SInt,

    // 16 bit integer formats
    R16_UNorm,
    R16G16_UNorm,
    R16G16B16_UNorm,
    R16G16B16A16_UNorm,
    R16_SNorm,
    R16G16_SNorm,
    R16G16B16_SNorm,
    R16G16B16A16_SNorm,
    R16_UInt,
    R16G16_UInt,
    R16G16B16_UInt,
    R16G16B16A16_UInt,
    R16_SInt,
    R16G16_SInt,
    R16G16B16_SInt,
    R16G16B16A16_SInt,

    // 32 bit integer formats
    R32_UInt,
    R32G32_UInt,
    R32G32B32_UInt,
    R32G32B32A32_UInt,
    R32_SInt,
    R32G32_SInt,
    R32G32B32_SInt,
    R32G32B32A32_SInt,

    // HDR formats
    R16_SFloat,
    R16G16_SFloat,
    R16G16B16_SFloat,
    R16G16B16A16_SFloat,
    R32_SFloat,
    R32G32_SFloat,
    R32G32B32_SFloat,
    R32G32B32A32_SFloat,

    // Luminance and Alpha format
    L8_UNorm,
    A8_UNorm,
    A16_UNorm,

    // BGR formats
    B8G8R8_SRGB,
    B8G8R8A8_SRGB,
    B8G8R8_UNorm,
    B8G8R8A8_UNorm,
    B8G8R8_SNorm,
    B8G8R8A8_SNorm,
    B8G8R8_UInt,
    B8G8R8A8_UInt,
    B8G8R8_SInt,
    B8G8R8A8_SInt,

    // 16 bit packed formats
    R4G4B4A4_UNormPack16,
    B4G4R4A4_UNormPack16,
    R5G6B5_UNormPack16,
    B5G6R5_UNormPack16,
    R5G5B5A1_UNormPack16,
    B5G5R5A1_UNormPack16,
    A1R5G5B5_UNormPack16,

    // Packed formats
    E5B9G9R9_UFloatPack32,
    B10G11R11_UFloatPack32,

    A2B10G10R10_UNormPack32,
    A2B10G10R10_UIntPack32,
    A2B10G10R10_SIntPack32,
    A2R10G10B10_UNormPack32,
    A2R10G10B10_UIntPack32,
    A2R10G10B10_SIntPack32,
    A2R10G10B10_XRSRGBPack32,
    A2R10G10B10_XRUNormPack32,
    R10G10B10_XRSRGBPack32,
    R10G10B10_XRUNormPack32,
    A10R10G10B10_XRSRGBPack32,
    A10R10G10B10_XRUNormPack32,

    // ARGB formats... TextureFormat legacy
    A8R8G8B8_SRGB,
    A8R8G8B8_UNorm,
    A32R32G32B32_SFloat,

    // Depth Stencil for formats
    D16_UNorm,
    D24_UNorm,
    D24_UNorm_S8_UInt,
    D32_SFloat,
    D32_SFloat_S8_Uint,
    S8_Uint,

    // Compression formats
    RGBA_DXT1_SRGB,
    RGBA_DXT1_UNorm,
    RGBA_DXT3_SRGB,
    RGBA_DXT3_UNorm,
    RGBA_DXT5_SRGB,
    RGBA_DXT5_UNorm,
    R_BC4_UNorm,
    R_BC4_SNorm,
    RG_BC5_UNorm,
    RG_BC5_SNorm,
    RGB_BC6H_UFloat,
    RGB_BC6H_SFloat,
    RGBA_BC7_SRGB,
    RGBA_BC7_UNorm,

    RGB_PVRTC_2Bpp_SRGB,
    RGB_PVRTC_2Bpp_UNorm,
    RGB_PVRTC_4Bpp_SRGB,
    RGB_PVRTC_4Bpp_UNorm,
    RGBA_PVRTC_2Bpp_SRGB,
    RGBA_PVRTC_2Bpp_UNorm,
    RGBA_PVRTC_4Bpp_SRGB,
    RGBA_PVRTC_4Bpp_UNorm,

    RGB_ETC_UNorm,
    RGB_ETC2_SRGB,
    RGB_ETC2_UNorm,
    RGB_A1_ETC2_SRGB,
    RGB_A1_ETC2_UNorm,
    RGBA_ETC2_SRGB,
    RGBA_ETC2_UNorm,

    R_EAC_UNorm,
    R_EAC_SNorm,
    RG_EAC_UNorm,
    RG_EAC_SNorm,

    RGBA_ASTC4X4_SRGB,
    RGBA_ASTC4X4_UNorm,
    RGBA_ASTC5X5_SRGB,
    RGBA_ASTC5X5_UNorm,
    RGBA_ASTC6X6_SRGB,
    RGBA_ASTC6X6_UNorm,
    RGBA_ASTC8X8_SRGB,
    RGBA_ASTC8X8_UNorm,
    RGBA_ASTC10X10_SRGB,
    RGBA_ASTC10X10_UNorm,
    RGBA_ASTC12X12_SRGB,
    RGBA_ASTC12X12_UNorm,

    // Video formats
    YUV2,

    // Automatic formats, back-end decides
    DepthAuto,
    ShadowAuto,
    VideoAuto,

    // ASTC hdr profile
    RGBA_ASTC4X4_UFloat,
    RGBA_ASTC5X5_UFloat,
    RGBA_ASTC6X6_UFloat,
    RGBA_ASTC8X8_UFloat,
    RGBA_ASTC10X10_UFloat,
    RGBA_ASTC12X12_UFloat,
    //kUnityRenderingExtFormatLast = kUnityRenderingExtFormatRGBA_ASTC12X12_UFloat, // Remove?
}

#[repr(C)]
pub struct UnityRenderingExtBeforeDrawCallParams {
    vertex_shader: *mut c_void,   // bound vertex shader (platform dependent)
    fragment_shader: *mut c_void, // bound fragment shader (platform dependent)
    geometry_shader: *mut c_void, // bound geometry shader (platform dependent)
    hull_shader: *mut c_void,     // bound hull shader (platform dependent)
    domain_shader: *mut c_void,   // bound domain shader (platform dependent)
    eye_index: i32,               // the index of the current stereo "eye" being currently rendered.
}

#[repr(C)]
pub struct UnityRenderingExtCustomBlitParams {
    source: UnityTextureID,         // source texture
    destination: UnityRenderBuffer, // destination surface
    command: u32, // command for the custom blit - could be any UnityRenderingExtCustomBlitCommands command or custom ones.
    command_param: u32, // custom parameters for the command
    command_flags: u32, // custom flags for the command
}

#[repr(C)]
pub struct UnityRenderingExtTextureUpdateParamsV1 {
    tex_data: *mut c_void, // source data for the texture update. Must be set by the plugin
    user_data: u32,        // user defined data. Set by the plugin
    texture_id: u32,       // texture ID of the texture to be updated.
    format: UnityRenderingExtTextureFormat, // format of the texture to be updated
    width: u32,            // width of the texture
    height: u32,           // height of the texture
    bpp: u32,              // texture bytes per pixel.
}

// Deprecated. Use UnityRenderingExtTextureUpdateParamsV2 and CommandBuffer.IssuePluginCustomTextureUpdateV2 instead.
// Only supports DX11, GLES, Metal
pub type UnityRenderingExtTextureUpdateParams = UnityRenderingExtTextureUpdateParamsV1;

// Type of the "data" parameter passed when callbacks registered with CommandBuffer.IssuePluginCustomTextureUpdateV2 are called.
// Supports DX11, GLES, Metal, and Switch (also possibly PS4, PSVita in the future)
#[repr(C)]
pub struct UnityRenderingExtTextureUpdateParamsV2 {
    tex_data: *mut c_void, // source data for the texture update. Must be set by the plugin
    texture_id: isize,     // texture ID of the texture to be updated.
    user_data: u32,        // user defined data. Set by the plugin
    format: UnityRenderingExtTextureFormat, // format of the texture to be updated
    width: u32,            // width of the texture
    height: u32,           // height of the texture
    bpp: u32,              // texture bytes per pixel.
}
