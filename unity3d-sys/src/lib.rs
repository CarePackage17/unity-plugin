pub mod unity_graphics;
pub mod unity_graphics_d3d11;
pub mod unity_interface;
pub mod unity_xr_trace;
pub mod unity_rendering_extensions;

// This is here so that the user of this crate can have Direct3D types instead of void*
// in unity_graphics_d3d11. Might not be the best way to do it, but works for now.
pub use winapi;