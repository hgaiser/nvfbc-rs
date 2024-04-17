#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "windows")] // d3d9helper.h
pub use windows::Win32::Graphics::Direct3D9::IDirect3DSurface9;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));