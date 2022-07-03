use std::cell::Cell;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use nvfbc_sys::{
	NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOWAIT,
	NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOFLAGS,
	NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOWAIT_IF_NEW_FRAME_READY
};

use crate::common::{
	Handle,
	check_ret,
	create_capture_session,
	create_handle,
	destroy_capture_session,
	destroy_handle,
	status,
};
use crate::{
	BufferFormat,
	Error,
	Status,
	CaptureType,
};

/// Different methods for capturing a frame.
pub enum CaptureMethod {
	NoWait = NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOWAIT as isize,
	NoWaitIfNewFrame = NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOFLAGS as isize,
	Blocking = NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOWAIT_IF_NEW_FRAME_READY as isize,
}

/// Contains information about a frame captured in a CUDA device.
///
/// The lifetime of this struct is tied to the lifetime of the SystemCapturer that captured this frame.
#[derive(Clone)]
pub struct SystemFrameInfo<'a> {
	/// Pointer to the frame that is grabbed.
	pub buffer: &'a [u8],
	/// Width of the captured frame.
	pub width: u32,
	/// Height of the captured frame.
	pub height: u32,
	/// Incremental ID of the current frame.
	///
	/// This can be used to identify a frame.
	pub current_frame: u32,
}

impl std::fmt::Debug for SystemFrameInfo<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SystemFrameInfo")
			.field("buffer", &self.buffer.as_ptr())
			.field("buffer_len", &self.buffer.len())
			.field("width", &self.width)
			.field("height", &self.height)
			.field("current_frame", &self.current_frame)
			.finish()
	}
}

/// Uses NVFBC to capture frames directly to system memory.
pub struct SystemCapturer {
	/// The nvfbc handle.
	handle: Handle,

	/// The pointer to the data buffer.
	///
	/// The pointer is stored in a [`Box`] because nvfbc will overwrite it when it re-allocates the buffer.
	/// If we stored the pointer directly in the struct, then moving the struct would cause nvfbc to write
	/// to an invalid memory address.
	///
	/// Since the writes to the pointer happen without the compiler knowing about it,
	/// the pointer is also stored in a [`Cell`].
	buffer: Box<Cell<*mut c_void>>,
}

impl SystemCapturer {
	/// Creates a new SystemCapturer object.
	///
	/// This also initializes a handle for the NVFBC API.
	pub fn new() -> Result<Self, Error> {
		let handle = create_handle()?;
		let self_ = Self { handle, buffer: Box::new(Cell::new(null_mut())) };
		Ok(self_)
	}

	/// Retrieve the status of NVFBC.
	pub fn status(&self) -> Result<Status, Error> {
		status(self.handle)
	}

	/// Start a capture session with the desired buffer format.
	pub fn start(&mut self, buffer_format: BufferFormat, fps: u32) -> Result<(), Error> {
		create_capture_session(
			self.handle,
			CaptureType::ToSystem,
			std::time::Duration::from_millis(1000 / fps as u64),
		)?;

		let mut params: nvfbc_sys::NVFBC_TOSYS_SETUP_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOSYS_SETUP_PARAMS_VER;
		params.eBufferFormat = buffer_format as u32;
		params.ppBuffer = self.buffer.as_ptr();
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToSysSetUp(self.handle, &mut params) })
	}

	/// Stop a capture session.
	pub fn stop(&self) -> Result<(), Error> {
		destroy_capture_session(self.handle)
	}

	/// Retrieve the next frame from the GPU.
	///
	/// Since NVFBC takes full control of the pointer to the buffer,
	/// only one frame is allowed to exist at the same time.
	/// This ensures that NVFBC does not change the buffer while there is access to it.
	///
	/// If this restriction would be lifted, there would be a risk of unsound behaviour.
	/// For example: calling next_frame() twice would overwrite the first buffer with the content of the second buffer.
	/// Changing resolution inbetween the two calls could lead to reading out of bounds memory.
	pub fn next_frame(&mut self, capture_method: CaptureMethod) -> Result<SystemFrameInfo, Error> {
		let mut frame_info: nvfbc_sys::NVFBC_FRAME_GRAB_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
		let mut params: nvfbc_sys::NVFBC_TOSYS_GRAB_FRAME_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOSYS_GRAB_FRAME_PARAMS_VER;
		params.dwFlags = capture_method as u32;
		params.pFrameGrabInfo = &mut frame_info;
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToSysGrabFrame(self.handle, &mut params) })?;
		let buffer_ptr = unsafe { self.buffer.as_ptr().read_volatile().cast() };
		let buffer = unsafe { std::slice::from_raw_parts(buffer_ptr, frame_info.dwByteSize as usize) };

		Ok(SystemFrameInfo {
			buffer,
			width: frame_info.dwWidth,
			height: frame_info.dwHeight,
			current_frame: frame_info.dwCurrentFrame,
		})
	}
}

impl Drop for SystemCapturer {
	fn drop(&mut self) {
		// TODO: Figure out why this crashes (nvfbc examples also fail here..)
		destroy_handle(self.handle).ok();
	}
}
