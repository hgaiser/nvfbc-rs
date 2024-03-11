use std::{env, path::PathBuf};

/*
 * The Nvidia Frame Buffer Capture SDK (NvFBC) was deprecated for Windows 10 1803.17134
 * However, the last supported version v7.1.1 still works with the latest GPU drivers.
 * It only works with enterprise Quadro cards, or with a driver patch for consumer cards.
 * Linux is still supported with the latest version v8.0.8
 *
 * The NVIDIA Capture SDK installer for Windows comes with the dynamic library and headers,
 * While for Linux only the header file is provided, and the dynamic library is expected
 * to be installed with the proprietary graphics driver. The header is included with this crate.
 */
fn main() {
	let include_path = if cfg!(target_os = "windows") {
		"C:\\Program Files (x86)\\NVIDIA Corporation\\NVIDIA Capture SDK\\inc\\NvFBC"
	} else if cfg!(target_os = "linux") {
		"." // Local header file
	} else {
		unimplemented!("Unsupported OS")
	};

	let library_name = if cfg!(target_os = "linux") {
		"nvidia-fbc" // System library name
	} else if cfg!(target_os = "windows") {
		if cfg!(target_arch = "x86_64") {
			"nvapi64"
		} else if cfg!(target_arch = "x86") {
			"nvapi"
		} else {
			unimplemented!("Unsupported Arch")
		}
	} else {
		unimplemented!("Unsupported OS")
	};

	let library_path = if cfg!(target_os = "linux") {
		"."
	} else if cfg!(target_os = "windows") {
		if cfg!(target_arch = "x86_64") {
			"C:\\Program Files (x86)\\NVIDIA Corporation\\NVIDIA Capture SDK\\lib\\NvAPI\\amd64"
		} else if cfg!(target_arch = "x86") {
			"C:\\Program Files (x86)\\NVIDIA Corporation\\NVIDIA Capture SDK\\lib\\NvAPI\\x86"
		} else {
			unimplemented!("Unsupported Arch")
		}
	} else {
		unimplemented!("Unsupported OS")
	};

	// Windows has multiple C++ header files
	let header_name = if cfg!(target_os = "linux") {
		"nvFBC.h"
	} else if cfg!(target_os = "windows") {
		"wrapper.h"
	} else {
		unimplemented!("Unsupported OS")
	};

	println!("cargo:rustc-link-search={library_path}");
	println!("cargo:rustc-link-lib={library_name}");
	println!("cargo:rerun-if-changed={header_name}");

	// Allowlist is transient while Blocklist is not
	// This results in less re-exported types from d3d9helper.h
	let bindings = bindgen::Builder::default()
		.header(header_name)
		.allowlist_file(".*nvFBC.h")
		.allowlist_file(".*nvFBCCuda.h")
		.allowlist_file(".*nvFBCToDx9Vid.h")
		.allowlist_file(".*nvFBCToSys.h")
		.blocklist_file(".*d3d9helper.h")
		.clang_args(["-I", include_path])
		.clang_args(["-x", "c++"])
		.clang_macro_fallback()
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
