//! This library contains a safe FFI for [NVFBC](https://developer.nvidia.com/capture-sdk) from NVIDIA.
//!
//! # Supported GPUs
//! As this uses a proprietary NVIDIA API, the supported devices are limited to NVIDIA GPUs.
//! Additionally, the NVFBC API is only officially supported on GRID, Tesla, or Quadro X2000+ GPUs.
//! Unofficial support is provided for other GPUs.
//!
//! # Supported capture types
//! Currently only CUDA and system (RAM) capture types are supported.
//!
//! # Example: Saving an image.
//! ```no_run
//! use nvfbc::{SystemCapturer, BufferFormat};
//! use nvfbc::system::CaptureMethod;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut capturer = SystemCapturer::new()?;
//!
//!     let status = capturer.status()?;
//!     println!("{:#?}", capturer.status()?);
//!     if !status.can_create_now {
//!         panic!("Can't create a system capture session.");
//!     }
//!
//!     capturer.start(BufferFormat::Rgb, 30)?;
//!
//!     let frame_info = capturer.next_frame(CaptureMethod::Blocking)?;
//!     println!("{:#?}", frame_info);
//!
//!     let image = image::ImageBuffer::<image::Rgb<u8>, &[u8]>::from_raw(
//!         frame_info.width,
//!         frame_info.height,
//!         frame_info.buffer,
//!     ).unwrap();
//!     image.save("frame.png")?;
//!     println!("Saved frame to 'frame.png'.");
//!
//!     capturer.stop()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Future work
//! Support for configuration is currently limited, to keep the code simple and concise.
//! Future releases will add more configuration options.

mod common;
pub mod cuda;
mod error;
pub mod system;
mod types;

pub use types::*;
pub use error::Error;
pub use cuda::CudaCapturer;
pub use system::SystemCapturer;
