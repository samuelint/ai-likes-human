use rayon::prelude::*;
use std::time::Instant;
use xcap::Monitor;
use base64::{Engine as _, engine::general_purpose};
use xcap::image::ImageOutputFormat;
use fast_image_resize::{Resizer, ResizeOptions, ResizeAlg, PixelType, FilterType, images::Image};
use image::{ImageBuffer, Rgba};


fn get_image_dimension(dimensions: (u32, u32), max_size: u32) -> (u32, u32) {
  let (src_width, src_height) = dimensions;

  let dst_width = if src_width > max_size {
    max_size
  } else {
    src_width
  };
  let dst_height = if src_height > max_size {
    max_size
  } else {
    src_height
  };

  let mut ratio = src_width as f64 / dst_width as f64;
  if ratio > src_height as f64 / dst_height as f64 {
    ratio = src_height as f64 / dst_height as f64;
  }

  let dst_width = (src_width as f64 / ratio).round() as u32;
  let dst_height = (src_height as f64 / ratio).round() as u32;

  (dst_width, dst_height)
}

fn resize_image(img: image::RgbaImage, max_size: u32) -> image::RgbaImage {
  let start = Instant::now();

  let (src_width, src_height) = img.dimensions();
  let src_image = Image::from_vec_u8(
    src_width,
    src_height,
    img.into_raw(),
    PixelType::U8x4,
  ).unwrap();


  let (dst_width, dst_height) = get_image_dimension((src_width, src_height), max_size);
  let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type());

  let mut resizer = Resizer::new();

  resizer.resize(&src_image, &mut dst_image, &ResizeOptions::new().resize_alg(
    ResizeAlg::SuperSampling(FilterType::Box, 10),
)).unwrap();

  let resized_img_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
    dst_width,
    dst_height,
    dst_image.into_vec(),
).unwrap();
  
  println!("Resize elapsed: {:?}", start.elapsed());

  return resized_img_buffer
}

#[tauri::command]
pub async fn capture_screen(max_size_arg: Option<u32>) -> Vec<String> {
  let max_size = max_size_arg.unwrap_or(1024);
  let monitors = Monitor::all().unwrap();

  let b64_images: Vec<String> = monitors.par_iter()
  .map(|monitor| {
      let img = monitor.capture_image().unwrap();
      let resized_img = resize_image(img, max_size);
     
      let mut png_data = std::io::Cursor::new(Vec::new());
      resized_img.write_to(&mut png_data, ImageOutputFormat::Png).unwrap();
  
      return format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(png_data.into_inner()));
  })
  .collect();

  b64_images
}
