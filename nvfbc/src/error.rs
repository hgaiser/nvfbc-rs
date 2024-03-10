use std::fmt;

#[cfg(target_os = "windows")]
use nvfbc_sys::_NVFBCRESULT as Code;
#[cfg(target_os = "linux")]
use nvfbc_sys::_NVFBCSTATUS as Code;

#[derive(Debug)]
pub struct Error {
	code: Code,
	message: Option<String>,
}

impl Error {
	pub fn new(code: Code, message: Option<String>) -> Self {
		Error { code, message }
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		#[rustfmt::skip]
		#[cfg(target_os = "linux")]
		let error_code_message = match self.code {
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_API_VERSION => "The API version between the client and the library is not compatible".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INTERNAL => "An internal error occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PARAM => "One or more of the parameter passed to the API call is invalid".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PTR => "One or more of the pointers passed to the API call is invalid".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_HANDLE => "The handle passed to the API call to identify the client is invalid".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MAX_CLIENTS => "The maximum number of threaded clients (10) of the same process has been reached".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_UNSUPPORTED => "The requested feature is not currently supported by the library".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_OUT_OF_MEMORY => "Unable to allocate enough memory to perform the requested operation".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_BAD_REQUEST => "The API call was not expected".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_X => "An unknown X error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GLX => "An unknown GLX error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GL => "An unknown OpenGL error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CUDA => "An unknown CUDA error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_ENCODER => "A hardware encoder error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CONTEXT => "An NVFBC context error has occurred".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MUST_RECREATE => "The capture session must be recreated".to_string(),
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_VULKAN => "A Vulkan error has occurred".to_string(),
			code => format!("Un unknown error code ({}) was returned", code),
		};

		#[rustfmt::skip]
		#[cfg(target_os = "windows")]
		let error_code_message = match self.code {
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_GENERIC => "Unexpected failure in NVFBC.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INVALID_PARAM => "One or more of the paramteres passed to NvFBC are invalid [This include NULL pointers].".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INVALIDATED_SESSION => "NvFBC session is invalid. Client needs to recreate session.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_PROTECTED_CONTENT => "Protected content detected. Capture failed.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_DRIVER_FAILURE => "GPU driver returned failure to process NvFBC command.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_CUDA_FAILURE => "CUDA driver returned failure to process NvFBC command.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_UNSUPPORTED => "API Unsupported on this version of NvFBC.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_HW_ENC_FAILURE => "HW Encoder returned failure to process NVFBC command.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INCOMPATIBLE_DRIVER => "NVFBC is not compatible with this version of the GPU driver.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_UNSUPPORTED_PLATFORM => "NVFBC is not supported on this platform.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_OUT_OF_MEMORY => "Failed to allocate memory.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INVALID_PTR => "A NULL pointer was passed.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INCOMPATIBLE_VERSION => "An API was called with a parameter struct that has an incompatible version. Check dwVersion field of paramter struct.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_OPT_CAPTURE_FAILURE => "Desktop Capture failed.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INSUFFICIENT_PRIVILEGES => "User doesn't have appropriate previlages.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INVALID_CALL => "NVFBC APIs called in wrong sequence.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_SYSTEM_ERROR => "Win32 error.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_INVALID_TARGET => "The target adapter idx can not be used for NVFBC capture. It may not correspond to an NVIDIA GPU, or may not be attached to desktop.".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_NVAPI_FAILURE => "NvAPI Error".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_DYNAMIC_DISABLE => "NvFBC is dynamically disabled. Cannot continue to capture".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_IPC_FAILURE => "NVFBC encountered an error in state management".to_string(),
			nvfbc_sys::_NVFBCRESULT_NVFBC_ERROR_CURSOR_CAPTURE_FAILURE => "Hardware cursor capture failed".to_string(),
			code => format!("Un unknown error code ({}) was returned", code),
		};

		if let Some(message) = &self.message {
			write!(f, "{}: {}", error_code_message, message)?;
		} else {
			write!(f, "{}", error_code_message)?;
		}

		Ok(())
	}
}

impl std::error::Error for Error {}
