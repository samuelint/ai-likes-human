use xcap::Monitor;
use base64::{Engine as _, engine::general_purpose};
use xcap::image::ImageOutputFormat;

#[tauri::command]
pub fn capture_screen() -> Vec<String> {
  let monitors = Monitor::all().unwrap();

  let mut b64_images: Vec<String> = Vec::new();
  for monitor in monitors {
    let image = monitor.capture_image().unwrap();

    let mut png_data = std::io::Cursor::new(Vec::new());
    image.write_to(&mut png_data, ImageOutputFormat::WebP).unwrap();
    let b64 = general_purpose::STANDARD.encode(png_data.into_inner());
    b64_images.push(b64);
  }
  b64_images
}