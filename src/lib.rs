mod image_utils;
use image_utils::{resize_image,chunk_image};
mod colors;
use colors::Color;
mod color_utils;
use color_utils::get_ldr_colors;
mod ldraw_utils;
use ldraw_utils::write_header;
use ldraw_utils::write_chunk;

use std::fs::File;

pub fn img2ldr(input_path:&str,output_path:&str,chunks_width:u32,chunks_height:u32) {
    let chunk_size:u32 = 16;
    let width:u32 = chunk_size*chunks_width;
    let height:u32 = chunk_size*chunks_height;
    let bytes = resize_image(input_path,width,height);

    let chunks:Vec<Vec<u8>> = chunk_image(&bytes,width,height,chunk_size);

    let mut file = File::create(output_path).unwrap();
    write_header(&mut file,output_path,output_path);
    for (i, chunk) in chunks.iter().enumerate() {
        let chunk_colors = get_ldr_colors(&Color::all(),&chunk);
        let max_x:u32 = width/chunk_size;
        let max_y:u32 = height/chunk_size;
        let offset_x:u32 = i as u32 % max_x;
        let offset_y:u32 = i as u32 / max_x;
        write_chunk(&mut file,&chunk_colors,chunk_size,offset_x,offset_y,max_x,max_y);
    }
}
