use std::fs::File;
use std::io::Write;
use crate::colors::Color;

pub fn write_header(buf : &mut File, title: &str ,filename: &str) {
    let _ = buf.write_all(format!("0 {}\n0 Name: {}\n0 Author: Kadsendino\n",title,filename).as_bytes());
}

pub fn write_chunk(buf : &mut File,chunk_colors : &Vec<Color>,chunk_size:u32,offset_x:u32,offset_y:u32,_max_x:u32,_max_y:u32) {
    let _ = buf.write_all(format!("1 0 {}.000000 -32.000000 -{}.000000 1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 65803.dat\n",20*chunk_size*offset_x+chunk_size*10-10,20*chunk_size*offset_y+chunk_size*10-10).as_bytes());
    for (i, c) in chunk_colors.iter().enumerate() {
        let pixel_x:u32 = (i % chunk_size as usize) as u32;
        let pixel_y:u32 = (i / 16) as u32;
        let _ = buf.write_all(format!("1 {} {}.000000 -40.000000 -{}.000000 1.000000 0.000000 0.000000 0.000000 1.000000 0.000000 0.000000 0.000000 1.000000 98138.dat\n",c.info().code,20*(chunk_size*offset_x+pixel_x),20*(chunk_size*offset_y+pixel_y)).as_bytes());
    }
    let _ = buf.write_all("0 STEP\n".as_bytes());
}
