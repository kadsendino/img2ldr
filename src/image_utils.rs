use fast_image_resize::Resizer;
use fast_image_resize::images::Image;
use fast_image_resize::IntoImageView;
use image::ImageReader;

pub fn resize_image(path: &str, dst_width: u32, dst_height: u32) -> Vec<u8> {
    let src_image = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap();

    let mut dst_image = Image::new(
        dst_width,
        dst_height,
        src_image.pixel_type().unwrap(),
    );

    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut dst_image, None).unwrap();

    dst_image.buffer().to_vec()
}

pub fn chunk_image(pixels: &Vec<u8>, width: u32, height: u32, chunk_size: u32) -> Vec<Vec<u8>> {
    let channels = pixels.len() as u32 / (width * height);
    let mut chunks = Vec::new();

    let cols = (width + chunk_size - 1) / chunk_size;
    let rows = (height + chunk_size - 1) / chunk_size;

    for row in 0..rows {
        for col in 0..cols {
            let mut chunk = Vec::new();
            let y_start = row * chunk_size;
            let y_end = ((row + 1) * chunk_size).min(height);
            let x_start = col * chunk_size;
            let x_end = ((col + 1) * chunk_size).min(width);

            for y in y_start..y_end {
                for x in x_start..x_end {
                    let pixel_index = ((y * width + x) * channels) as usize;
                    for c in 0..channels as usize {
                        chunk.push(pixels[pixel_index + c]);
                    }
                }
            }
            chunks.push(chunk);
        }
    }
    chunks
}
