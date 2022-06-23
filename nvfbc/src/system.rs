use std::cell::Cell;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

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
	SystemFrameInfo
};

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
	pub fn new() -> Result<Self, Error> {
		let handle = create_handle()?;
		let self_ = Self { handle, buffer: Box::new(Cell::new(null_mut())) };
		Ok(self_)
	}

	pub fn status(&self) -> Result<Status, Error> {
		status(self.handle)
	}

	pub fn start(&mut self, buffer_format: BufferFormat) -> Result<(), Error> {
		create_capture_session(self.handle, CaptureType::ToSystem)?;

		let mut params: nvfbc_sys::NVFBC_TOSYS_SETUP_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOSYS_SETUP_PARAMS_VER;
		params.eBufferFormat = buffer_format as u32;
		params.ppBuffer = self.buffer.as_ptr();
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToSysSetUp(self.handle, &mut params) })
	}

	pub fn stop(&self) -> Result<(), Error> {
		destroy_capture_session(self.handle)
	}

	pub fn next_frame(&mut self) -> Result<SystemFrameInfo, Error> {
		let mut frame_info: nvfbc_sys::NVFBC_FRAME_GRAB_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
		let mut params: nvfbc_sys::NVFBC_TOSYS_GRAB_FRAME_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOSYS_GRAB_FRAME_PARAMS_VER;
		params.dwFlags = nvfbc_sys::NVFBC_TOSYS_GRAB_FLAGS_NVFBC_TOSYS_GRAB_FLAGS_NOFLAGS;
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
