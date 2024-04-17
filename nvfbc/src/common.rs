use std::{ffi::CStr, mem::MaybeUninit, os::raw::c_uint};

use nvfbc_sys::{NVFBC_SESSION_HANDLE, _NVFBCSTATUS_NVFBC_SUCCESS as SUCCESS};

use crate::{CaptureType, Error, Status};

pub type Handle = NVFBC_SESSION_HANDLE;

pub(crate) fn check_ret(handle: Handle, ret: nvfbc_sys::_NVFBCSTATUS) -> Result<(), Error> {
	if ret != SUCCESS {
		return Err(Error::new(ret, get_last_error(handle)));
	}
	Ok(())
}

pub(crate) fn create_handle() -> Result<nvfbc_sys::NVFBC_SESSION_HANDLE, Error> {
	const MAGIC_PRIVATE_DATA: [u32; 4] = [0xAEF57AC5, 0x401D1A39, 0x1B856BBE, 0x9ED0CEBA];

	let mut params: nvfbc_sys::_NVFBC_CREATE_HANDLE_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
	params.dwVersion = nvfbc_sys::NVFBC_CREATE_HANDLE_PARAMS_VER;
	params.privateData = MAGIC_PRIVATE_DATA.as_ptr() as _;
	params.privateDataSize = std::mem::size_of_val(&MAGIC_PRIVATE_DATA) as u32;

	let mut handle = 0;
	let ret = unsafe { nvfbc_sys::NvFBCCreateHandle(&mut handle, &mut params) };
	if ret != SUCCESS {
		return Err(Error::new(ret, None));
	}

	Ok(handle)
}

pub(crate) fn destroy_handle(handle: Handle) -> Result<(), Error> {
	let mut params: nvfbc_sys::_NVFBC_DESTROY_HANDLE_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
	params.dwVersion = nvfbc_sys::NVFBC_DESTROY_HANDLE_PARAMS_VER;
	check_ret(handle, unsafe { nvfbc_sys::NvFBCDestroyHandle(handle, &mut params) })
}

pub(crate) fn get_last_error(handle: Handle) -> Option<String> {
	let error = unsafe { nvfbc_sys::NvFBCGetLastErrorStr(handle) };
	let error = unsafe { CStr::from_ptr(error) };
	error.to_str().ok().map(|e| e.to_string())
}

pub(crate) fn status(handle: Handle) -> Result<Status, Error> {
	let mut params: nvfbc_sys::_NVFBC_GET_STATUS_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
	params.dwVersion = nvfbc_sys::NVFBC_GET_STATUS_PARAMS_VER;
	check_ret(handle, unsafe { nvfbc_sys::NvFBCGetStatus(handle, &mut params) })?;
	Ok(params.into())
}

pub(crate) fn create_capture_session(
	handle: Handle,
	capture_type: CaptureType,
	sampling_rate: std::time::Duration,
) -> Result<(), Error> {
	let mut params: nvfbc_sys::_NVFBC_CREATE_CAPTURE_SESSION_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
	params.dwVersion = nvfbc_sys::NVFBC_CREATE_CAPTURE_SESSION_PARAMS_VER;
	params.eCaptureType = capture_type as c_uint;
	params.bWithCursor = nvfbc_sys::_NVFBC_BOOL_NVFBC_TRUE;
	params.frameSize = nvfbc_sys::NVFBC_SIZE { w: 0, h: 0 };
	params.eTrackingType = nvfbc_sys::NVFBC_TRACKING_TYPE_NVFBC_TRACKING_DEFAULT;
	params.dwSamplingRateMs = sampling_rate.as_millis() as u32;
	check_ret(handle, unsafe {
		nvfbc_sys::NvFBCCreateCaptureSession(handle, &mut params)
	})
}

pub(crate) fn destroy_capture_session(handle: Handle) -> Result<(), Error> {
	let mut params: nvfbc_sys::_NVFBC_DESTROY_CAPTURE_SESSION_PARAMS = unsafe { MaybeUninit::zeroed().assume_init() };
	params.dwVersion = nvfbc_sys::NVFBC_DESTROY_CAPTURE_SESSION_PARAMS_VER;
	check_ret(handle, unsafe {
		nvfbc_sys::NvFBCDestroyCaptureSession(handle, &mut params)
	})
}
