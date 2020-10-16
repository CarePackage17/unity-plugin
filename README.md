# Rust Unity Plugin

This showcases a [Unity Native Plugin](https://docs.unity3d.com/Manual/NativePlugins.html) written in Rust that clears the screen using Direct3D 11 (and logs a bunch of messages when loading for the first time).

## Code Organization
`unity3d-sys` contains raw, unsafe bindings to the native Unity API.  
`src/lib.rs` uses those to talk to Unity and clear the screen with a red color.

Functionality has been tested with Unity `2019.4.1f1` (LTS).