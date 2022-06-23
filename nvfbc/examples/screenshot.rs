use image::Rgb;
use nvfbc::{SystemCapturer, BufferFormat, Error};

fn main() -> Result<(), Error> {
	let mut capturer = SystemCapturer::new()?;

	println!("{:#?}", capturer.status()?);

	capturer.start(BufferFormat::Rgb)?;

	let frame_info = capturer.next_frame()?;
	println!("{:#?}", frame_info);

	let image = image::ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(
		frame_info.width,
		frame_info.height,
		frame_info.buffer,
	).unwrap();
	image.save("frame.png").unwrap();
	println!("Saved frame to 'frame.png'.");

	capturer.stop()?;

	Ok(())
}
