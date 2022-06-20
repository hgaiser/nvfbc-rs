use std::fmt;

#[derive(Debug)]
pub struct Error {
	code: u32,
	message: Option<String>,
}

impl Error {
	pub fn new(code: u32, message: Option<String>) -> Self {
		Error { code, message }
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

		if let Some(message) = &self.message {
			write!(f, "{}: {}", error_code_message, message)?;
		} else {
			write!(f, "{}", error_code_message)?;
		}

		Ok(())
	}
}
