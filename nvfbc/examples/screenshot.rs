use std::slice::from_raw_parts;

use image::Rgb;
use nvfbc::{SystemCapturer, BufferFormat, Error};

fn main() -> Result<(), Error> {
	let mut capturer = SystemCapturer::new()?;

	println!("{:#?}", capturer.status()?);

	capturer.start(BufferFormat::Rgb)?;

	let frame_info = capturer.next()?;
	println!("{:#?}", frame_info);

	let image = image::ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(
		frame_info.width,
		frame_info.height,
		unsafe { from_raw_parts(frame_info.buffer as *const u8, frame_info.byte_size) }
	).unwrap();
	image.save("frame.png").unwrap();
	println!("Saved frame to 'frame.png'.");

	capturer.stop()?;

	Ok(())
}
