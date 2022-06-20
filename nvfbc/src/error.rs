use nvfbc_sys::NVFBCSTATUS;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("internal nvfbc error")]
	InternalError(NvFbcSysError, Option<String>),

	#[error("a utf-8 error occured")]
	Utf8(#[from] std::str::Utf8Error),
}

#[derive(Error, Debug)]
pub enum NvFbcSysError {
	/// This indicates that the API version between the client and the library is not compatible.
	#[error("invalid API version")]
	ApiVersion = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_API_VERSION as isize,
	/// An internal error occurred.
	#[error("internal error occurred")]
	Internal = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INTERNAL as isize,
	/// This indicates that one or more of the parameter passed to the API call is invalid.
	#[error("received invalid parameter")]
	InvalidParam = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PARAM as isize,
	/// This indicates that one or more of the pointers passed to the API call is invalid.
	#[error("received invalid pointer")]
	InvalidPtr = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PTR as isize,
	/// This indicates that the handle passed to the API call to identify the client is invalid.
	#[error("received invalid handle")]
	InvalidHandle = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_HANDLE as isize,
	/// This indicates that the maximum number of threaded clients of the same process has been reached.
	/// The limit is 10 threads per process. There is no limit on the number of process.
	#[error("reached maximum number of threaded clients")]
	MaxClients = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MAX_CLIENTS as isize,
	/// This indicates that the requested feature is not currently supported by the library.
	#[error("the requested feature is unsupported")]
	Unsupported = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_UNSUPPORTED as isize,
	/// This indicates that the API call failed because it was unable to allocate
	/// enough memory to perform the requested operation.
	#[error("unable to allocate enough memory")]
	OutOfMemory = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_OUT_OF_MEMORY as isize,
	/// This indicates that the API call was not expected.
	/// This happens when API calls are performed in a wrong order,
	/// such as trying to capture a frame prior to creating a new capture session;
	/// or trying to set up a capture to video memory although a capture session to system memory was created.
	#[error("received unexpected API call")]
	BadRequest = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_BAD_REQUEST as isize,
	/// This indicates an X error, most likely meaning that the X server has
	/// been terminated.  When this error is returned, the only resort is to
	/// create another FBC handle using NvFBCCreateHandle().
	///
	/// The previous handle should still be freed with NvFBCDestroyHandle(), but
	/// it might leak resources, in particular X, GLX, and GL resources since
	/// it is no longer possible to communicate with an X server to free them
	/// through the driver.
	///
	/// The best course of action to eliminate this potential leak is to close
	/// the OpenGL driver, close the forked process running the capture, or
	/// restart the application.
	#[error("an X error occured")]
	X = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_X as isize,
	/// This indicates a GLX error.
	#[error("a GLX error occured")]
	Glx = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GLX as isize,
	/// This indicates an OpenGL error.
	#[error("an OpenGL error occured")]
	Gl = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GL as isize,
	/// This indicates a CUDA error.
	#[error("a CUDA error occured")]
	Cuda = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CUDA as isize,
	/// This indicates a HW encoder error.
	#[error("an encoder error occured")]
	Encoder = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_ENCODER as isize,
	/// This indicates an NvFBC context error.
	#[error("an NvFBC context error occured")]
	Context = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CONTEXT as isize,
	/// This indicates that the application must recreate the capture session.
	///
	/// This error can be returned if a modeset event occurred while capturing
	/// frames, and NVFBC_CREATE_HANDLE_PARAMS::bDisableAutoModesetRecovery
	/// was set to NVFBC_TRUE.
	#[error("must recreate capture session")]
	MustRecreate = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MUST_RECREATE as isize,
	/// This indicates a Vulkan error.
	#[error("a vulkan error occured")]
	Vulkan = nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_VULKAN as isize,
}

impl From<NVFBCSTATUS> for NvFbcSysError {
	fn from(error: NVFBCSTATUS) -> Self {
		match error {
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_API_VERSION => NvFbcSysError::ApiVersion,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INTERNAL => NvFbcSysError::Internal,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PARAM => NvFbcSysError::InvalidParam,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_PTR => NvFbcSysError::InvalidPtr,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_INVALID_HANDLE => NvFbcSysError::InvalidHandle,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MAX_CLIENTS => NvFbcSysError::MaxClients,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_UNSUPPORTED => NvFbcSysError::Unsupported,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_OUT_OF_MEMORY => NvFbcSysError::OutOfMemory,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_BAD_REQUEST => NvFbcSysError::BadRequest,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_X => NvFbcSysError::X,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GLX => NvFbcSysError::Glx,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_GL => NvFbcSysError::Gl,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CUDA => NvFbcSysError::Cuda,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_ENCODER => NvFbcSysError::Encoder,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_CONTEXT => NvFbcSysError::Context,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_MUST_RECREATE => NvFbcSysError::MustRecreate,
			nvfbc_sys::_NVFBCSTATUS_NVFBC_ERR_VULKAN => NvFbcSysError::Vulkan,
			_ => panic!("Unknown error code: {}", error),
		}
	}
}
