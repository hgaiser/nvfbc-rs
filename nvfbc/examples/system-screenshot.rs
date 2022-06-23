use std::error::Error;
use image::Rgb;
use nvfbc::{SystemCapturer, BufferFormat};

fn main() -> Result<(), Box<dyn Error>> {
	let mut capturer = SystemCapturer::new()?;

	let status = capturer.status()?;
	println!("{:#?}", capturer.status()?);
	if !status.can_create_now {
		panic!("Can't create a system capture session.");
	}

	capturer.start(BufferFormat::Rgb)?;

	let frame_info = capturer.next_frame()?;
	println!("{:#?}", frame_info);

	let image = image::ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(
		frame_info.width,
		frame_info.height,
		frame_info.buffer,
	).unwrap();
	image.save("frame.png")?;
	println!("Saved frame to 'frame.png'.");

	capturer.stop()?;

	Ok(())
}
