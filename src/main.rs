use img2ldr::img2ldr;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut input = String::new();
    let mut output = String::new();
    let mut chunk_w: u32 = 3;
    let mut chunk_h: u32 = 3;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--input" => {
                i += 1;
                input = args[i].clone();
            }
            "-o" | "--output" => {
                i += 1;
                output = args[i].clone();
            }
            "-w" | "--width" => {
                i += 1;
                chunk_w = args[i].parse().expect("Width must be a number");
            }
            "-h" | "--height" => {
                i += 1;
                chunk_h = args[i].parse().expect("Height must be a number");
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if input.is_empty() {
        eprintln!("Error: input file is required. Use -i <file>");
        std::process::exit(1);
    }
    if output.is_empty() {
        eprintln!("Error: output file is required. Use -o <file>");
        std::process::exit(1);
    }

    img2ldr(&input, &output, chunk_w, chunk_h);
}
