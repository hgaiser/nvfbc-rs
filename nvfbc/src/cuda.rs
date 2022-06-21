use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use crate::{
	BufferFormat,
	CaptureType,
	CudaFrameInfo,
	Error,
	Status,
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

pub struct CudaCapturer {
	handle: Handle,
}

impl CudaCapturer {
	pub fn new() -> Result<Self, Error> {
		let handle = create_handle()?;
		let self_ = Self { handle };
		Ok(self_)
	}

	pub fn status(&self) -> Result<Status, Error> {
		status(self.handle)
	}

	pub fn start(&self, buffer_format: BufferFormat) -> Result<(), Error> {
		create_capture_session(self.handle, CaptureType::SharedCuda)?;

		let mut params: nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS_VER;
		params.eBufferFormat = buffer_format as u32;
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToCudaSetUp(self.handle, &mut params) })
	}

	pub fn stop(&self) -> Result<(), Error> {
		destroy_capture_session(self.handle)
	}

	pub fn next_frame(&mut self) -> Result<CudaFrameInfo, Error> {
		let device_buffer: *mut c_void = null_mut();
		let mut frame_info: nvfbc_sys::NVFBC_FRAME_GRAB_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
		let mut params: nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS_VER;
		params.dwFlags = nvfbc_sys::NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOFLAGS;
		params.pFrameGrabInfo = &mut frame_info;
		params.pCUDADeviceBuffer = device_buffer;
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToCudaGrabFrame(self.handle, &mut params) })?;

		Ok(CudaFrameInfo {
			device_buffer,
			width: frame_info.dwWidth,
			height: frame_info.dwHeight,
			byte_size: frame_info.dwByteSize,
			current_frame: frame_info.dwCurrentFrame,
		})
	}
}

impl Drop for CudaCapturer {
	fn drop(&mut self) {
		// TODO: Figure out why this crashes (nvfbc examples also fail here..)
		destroy_handle(self.handle).ok();
	}
}
