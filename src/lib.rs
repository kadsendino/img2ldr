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
use std::io::Write;

pub fn img2ldr() {
    let chunk_size:u32 = 16;
    let width:u32 = chunk_size*6;
    let height:u32 = chunk_size*4;
    let bytes = resize_image("data/kurzgesagt.jpg",width,height);

    //resized image
    let output_path:&str = "data/kurzgesagt_resized.pgm";
    let mut file = File::create(output_path).unwrap();
    file.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes()).unwrap();
    file.write_all(&bytes).unwrap();

    //chunked image
    let chunks:Vec<Vec<u8>> = chunk_image(&bytes,width,height,chunk_size);

    let output_path:&str = "data/kurzgesagt_chunk.pgm";
    let mut file = File::create(output_path).unwrap();
    file.write_all(format!("P6\n{} {}\n255\n", chunk_size, chunk_size).as_bytes()).unwrap();
    file.write_all(&(chunks[5])).unwrap();

    // transform chunk to brick colors
    let brick_colors:Vec<Color> = get_ldr_colors(&Color::all(),&(chunks[5]));
    let mut ldr_bytes: Vec<u8> = Vec::new();
    for brick in &brick_colors {
        let hex = brick.info().value.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        ldr_bytes.push(r);
        ldr_bytes.push(g);
        ldr_bytes.push(b);
    }

    let mut file = File::create("data/kurzgesagt_chunk_ldr.pgm").unwrap();
    file.write_all(format!("P6\n{} {}\n255\n", chunk_size, chunk_size).as_bytes()).unwrap();
    file.write_all(&ldr_bytes).unwrap();

    let ldraw_file = "data/test.ldr";
    let mut file = File::create(ldraw_file).unwrap();
    write_header(&mut file,ldraw_file,ldraw_file);
    for (i, chunk) in chunks.iter().enumerate() {
        let chunk_colors = get_ldr_colors(&Color::all(),&chunk);
        let max_x:u32 = width/chunk_size;
        let max_y:u32 = height/chunk_size;
        let offset_x:u32 = i as u32 % max_x;
        let offset_y:u32 = i as u32 / max_x;
        write_chunk(&mut file,&chunk_colors,chunk_size,offset_x,offset_y,max_x,max_y);
    }
}
