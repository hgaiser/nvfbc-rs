use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use nvfbc_sys::{
	NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOWAIT,
	NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOFLAGS,
	NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOWAIT_IF_NEW_FRAME_READY
};

use crate::{
	BufferFormat,
	CaptureType,
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

pub enum CaptureMethod {
	/// Capturing does not wait for a new frame nor a mouse move.
	///
	/// It is therefore possible to capture the same frame multiple times.
	/// When this occurs, the current_frame parameter of the
	/// CudaFrameInfo struct is not incremented.
	NoWait = NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOWAIT as isize,

	/// Similar to NoWait, except that the capture will not wait if there
	/// is already a frame available that the client has never seen yet.
	NoWaitIfNewFrame = NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOWAIT_IF_NEW_FRAME_READY as isize,

	/// Capturing waits for a new frame or mouse move.
	Blocking = NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOFLAGS as isize,
}

/// Contains information about a frame captured in a CUDA device.
#[derive(Copy, Clone)]
pub struct CudaFrameInfo {
	/// Address of the CUDA buffer where the frame is grabbed.
	///
	/// Note that this an address in CUDA memory, not in system memory.
	pub device_buffer: usize,
	/// Size of the frame in bytes.
	pub device_buffer_len: u32,
	/// Width of the captured frame.
	pub width: u32,
	/// Height of the captured frame.
	pub height: u32,
	/// Incremental ID of the current frame.
	///
	/// This can be used to identify a frame.
	pub current_frame: u32,
}

impl std::fmt::Debug for CudaFrameInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CudaFrameInfo")
			.field("device_buffer", &(&self.device_buffer as *const usize))
			.field("device_buffer_len", &self.device_buffer_len)
			.field("width", &self.width)
			.field("height", &self.height)
			.field("current_frame", &self.current_frame)
			.finish()
	}
}

/// Uses NVFBC to capture frames in the form of a CUDA device pointer.
pub struct CudaCapturer {
	/// A handle to the internal NVFBC instance used for FFI interaction.
	handle: Handle,
}

impl CudaCapturer {
	/// Create a new CUDA capture object.
	///
	/// CUDA must be initialized before creating this object.
	pub fn new() -> Result<Self, Error> {
		Ok(Self { handle: create_handle()? })
	}

	/// Retrieve the status of NVFBC.
	pub fn status(&self) -> Result<Status, Error> {
		status(self.handle)
	}

	/// Start a capture session with the desired buffer format.
	pub fn start(&self, buffer_format: BufferFormat, sampling_rate: std::time::Duration) -> Result<(), Error> {
		create_capture_session(self.handle, CaptureType::SharedCuda, sampling_rate)?;

		let mut params: nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS_VER;
		params.eBufferFormat = buffer_format as u32;
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToCudaSetUp(self.handle, &mut params) })
	}

	/// Stop a capture session.
	pub fn stop(&self) -> Result<(), Error> {
		destroy_capture_session(self.handle)
	}

	/// Retrieve the next frame from the GPU.
	pub fn next_frame(&mut self, capture_method: CaptureMethod) -> Result<CudaFrameInfo, Error> {
		let mut device_buffer: *mut c_void =  null_mut();
		let mut frame_info: nvfbc_sys::NVFBC_FRAME_GRAB_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
		let mut params: nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS_VER;
		params.dwFlags = capture_method as u32;
		params.pFrameGrabInfo = &mut frame_info;
		params.pCUDADeviceBuffer = &mut device_buffer as *mut _ as *mut c_void;
		check_ret(self.handle, unsafe { nvfbc_sys::NvFBCToCudaGrabFrame(self.handle, &mut params) })?;

		Ok(CudaFrameInfo {
			device_buffer: device_buffer as usize,
			device_buffer_len: frame_info.dwByteSize,
			width: frame_info.dwWidth,
			height: frame_info.dwHeight,
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
