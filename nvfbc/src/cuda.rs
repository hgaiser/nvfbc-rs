use std::{ffi::{CStr, c_void}, mem::MaybeUninit, ptr::null_mut, os::raw::c_uint};

use nvfbc_sys::_NVFBCSTATUS_NVFBC_SUCCESS as SUCCESS;

use crate::{BufferFormat, Error, Status, CaptureType, CudaFrameInfo};

pub struct CudaFbc {
	handle: nvfbc_sys::NVFBC_SESSION_HANDLE,
}

impl CudaFbc {
	pub fn new(buffer_format: BufferFormat) -> Result<Self, Error> {
		let handle = Self::create_handle()?;

		let self_ = Self { handle };
		self_.setup(buffer_format)?;

		Ok(self_)
	}

	fn create_handle() -> Result<nvfbc_sys::NVFBC_SESSION_HANDLE, Error> {
		let mut params: nvfbc_sys::_NVFBC_CREATE_HANDLE_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_CREATE_HANDLE_PARAMS_VER;
		let mut handle = 0;
		let ret = unsafe { nvfbc_sys::NvFBCCreateHandle(
			&mut handle,
			&mut params
		)};
		if ret != SUCCESS {
			return Err(Error::new(ret, None));
		}

		Ok(handle)
	}

	fn destroy_handle(&self) -> Result<(), Error> {
		let mut params: nvfbc_sys::_NVFBC_DESTROY_HANDLE_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_DESTROY_HANDLE_PARAMS_VER;
		let ret = unsafe { nvfbc_sys::NvFBCDestroyHandle(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(())
	}

	fn get_last_error(&self) -> Option<String> {
		let error = unsafe { nvfbc_sys::NvFBCGetLastErrorStr(self.handle) };
		let error = unsafe { CStr::from_ptr(error) };
		error.to_str().ok().map(|e| e.to_string())
	}

	fn setup(&self, buffer_format: BufferFormat) -> Result<(), Error> {
		let mut params: nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_SETUP_PARAMS_VER;
		params.eBufferFormat = buffer_format as u32;
		let ret = unsafe { nvfbc_sys::NvFBCToCudaSetUp(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(())
	}

	pub fn status(&self) -> Result<Status, Error> {
		let mut params: nvfbc_sys::_NVFBC_GET_STATUS_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_GET_STATUS_PARAMS_VER;
		let ret = unsafe { nvfbc_sys::NvFBCGetStatus(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(params.into())
	}

	pub fn start(&self, capture_type: CaptureType) -> Result<(), Error> {
		let mut params: nvfbc_sys::_NVFBC_CREATE_CAPTURE_SESSION_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_CREATE_CAPTURE_SESSION_PARAMS_VER;
		params.eCaptureType = capture_type as c_uint;
		params.bWithCursor = nvfbc_sys::_NVFBC_BOOL_NVFBC_TRUE;
		params.frameSize = nvfbc_sys::NVFBC_SIZE { w: 0, h: 0 };
		params.eTrackingType = nvfbc_sys::NVFBC_TRACKING_TYPE_NVFBC_TRACKING_DEFAULT;
		let ret = unsafe { nvfbc_sys::NvFBCCreateCaptureSession(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(())
	}

	pub fn stop(&self) -> Result<(), Error> {
		let mut params: nvfbc_sys::_NVFBC_DESTROY_CAPTURE_SESSION_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_DESTROY_CAPTURE_SESSION_PARAMS_VER;
		let ret = unsafe { nvfbc_sys::NvFBCDestroyCaptureSession(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(())
	}

	pub fn frame(&self) -> Result<CudaFrameInfo, Error> {
		let mut device_buffer: *mut c_void = null_mut();
		let mut frame_info: nvfbc_sys::NVFBC_FRAME_GRAB_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
		let mut params: nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
		params.dwVersion = nvfbc_sys::NVFBC_TOCUDA_GRAB_FRAME_PARAMS_VER;
		params.dwFlags = nvfbc_sys::NVFBC_TOCUDA_FLAGS_NVFBC_TOCUDA_GRAB_FLAGS_NOWAIT;
		params.pFrameGrabInfo = &mut frame_info;
		params.pCUDADeviceBuffer = &mut device_buffer as *mut _ as *mut c_void;
		let ret = unsafe { nvfbc_sys::NvFBCToCudaGrabFrame(self.handle, &mut params) };
		if ret != SUCCESS {
			return Err(Error::new(ret, self.get_last_error()));
		}

		Ok(CudaFrameInfo {
			device_buffer,
			width: frame_info.dwWidth,
			height: frame_info.dwHeight,
			byte_size: frame_info.dwByteSize,
			current_frame: frame_info.dwCurrentFrame,
		})
	}
}

impl Drop for CudaFbc {
	fn drop(&mut self) {
		self.stop().unwrap();
		// TODO: Figure out why this crashes (nvfbc examples also fail here..)
		self.destroy_handle().unwrap();
	}
}
