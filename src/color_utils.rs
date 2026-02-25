use crate::colors::Color;

fn get_ldr_color(available_colors: &[Color], pixel: (u8, u8, u8)) -> Color {
    let (r, g, b) = pixel;

    available_colors
        .iter()
        .min_by_key(|color| {
            let hex = color.info().value.trim_start_matches('#');

            let cr = i32::from_str_radix(&hex[0..2], 16).unwrap();
            let cg = i32::from_str_radix(&hex[2..4], 16).unwrap();
            let cb = i32::from_str_radix(&hex[4..6], 16).unwrap();

            let dr = cr - r as i32;
            let dg = cg - g as i32;
            let db = cb - b as i32;

            dr * dr + dg * dg + db * db
        })
        .unwrap()
        .clone()
}

pub fn get_ldr_colors(available_colors: &Vec<Color>, pixels: &[u8]) -> Vec<Color> {
    pixels
        .chunks_exact(3)
        .map(|chunk| {
            let pixel = (chunk[0], chunk[1], chunk[2]);
            get_ldr_color(available_colors, pixel)
        })
        .collect()
}
