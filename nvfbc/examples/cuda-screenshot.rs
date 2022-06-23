use std::error::Error;

use image::{Rgb, ImageBuffer};
use nvfbc::{BufferFormat, CudaCapturer};
use rustacuda::{CudaFlags, device::Device, context::{Context, ContextFlags}, prelude::{DeviceBuffer, CopyDestination}, memory::LockedBuffer};
use rustacuda_core::DevicePointer;

fn main() -> Result<(), Box<dyn Error>> {
	// Initialize the CUDA API
	rustacuda::init(CudaFlags::empty())?;

	// Get the first device
	let device = Device::get_device(0)?;

	// Create a context associated to this device
	let _context = Context::create_and_push(
		ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

	// Create a capturer that captures to CUDA context.
	let mut capturer = CudaCapturer::new()?;

	let status = capturer.status()?;
	println!("get_status: {:#?}", status);
	if !status.can_create_now {
		panic!("Can't create a CUDA capture session.");
	}

	capturer.start(BufferFormat::Rgb)?;

	let frame_info = capturer.next_frame()?;
	println!("{:#?}", frame_info);

	// Wrap the buffer in GPU memory.
	let device_buffer = unsafe { DeviceBuffer::from_raw_parts(
		DevicePointer::wrap(frame_info.device_buffer as *mut u8),
		frame_info.byte_size as usize,
	) };

	// Create a page locked buffer to avoid unnecessary copying.
	// See https://docs.rs/rustacuda/latest/rustacuda/memory/index.html#page-locked-host-memory for more information.
	let mut data: LockedBuffer<u8> = unsafe { LockedBuffer::uninitialized(frame_info.byte_size as usize) }?;

	// Copy device memory to host memory and wrap it as an image.
	device_buffer.copy_to(&mut data)?;
	let slice = data.as_slice();
	let frame = ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(frame_info.width, frame_info.height, slice).unwrap();
	frame.save("frame.png")?;

	// TODO: Find a better way to avoid a double free.
	std::mem::forget(device_buffer);

	capturer.stop()?;

	Ok(())
}
