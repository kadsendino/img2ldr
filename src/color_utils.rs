use crate::colors::Color;

fn rgb_to_lab(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let linearize = |c: u8| {
        let c = c as f64 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055_f64).powf(2.4) }
    };

    let r = linearize(r);
    let g = linearize(g);
    let b = linearize(b);

    let x = r * 0.4124564 + g * 0.3575761 + b * 0.1804375;
    let y = r * 0.2126729 + g * 0.7151522 + b * 0.0721750;
    let z = r * 0.0193339 + g * 0.1191920 + b * 0.9503041;

    let f = |t: f64| {
        if t > 0.008856 { t.cbrt() } else { 7.787 * t + 16.0 / 116.0 }
    };

    let fx = f(x / 0.95047);
    let fy = f(y / 1.00000);
    let fz = f(z / 1.08883);

    (116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
}

fn ciede2000(l1: f64, a1: f64, b1: f64, l2: f64, a2: f64, b2: f64) -> f64 {
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    let c_bar = (c1 + c2) / 2.0;
    let c_bar7 = c_bar.powi(7);
    let c_bar7_25_7 = c_bar7 + 25_f64.powi(7);

    let g = 0.5 * (1.0 - (c_bar7 / c_bar7_25_7).sqrt());
    let a1p = a1 + a1 / 2.0 * (1.0 - (c_bar7 / c_bar7_25_7).sqrt());
    let a2p = a2 + a2 / 2.0 * (1.0 - (c_bar7 / c_bar7_25_7).sqrt());
    let _ = g; // used implicitly via a1p/a2p

    let c1p = (a1p * a1p + b1 * b1).sqrt();
    let c2p = (a2p * a2p + b2 * b2).sqrt();

    let h1p = if a1p == 0.0 && b1 == 0.0 {
        0.0
    } else {
        let h = b1.atan2(a1p).to_degrees();
        if h < 0.0 { h + 360.0 } else { h }
    };

    let h2p = if a2p == 0.0 && b2 == 0.0 {
        0.0
    } else {
        let h = b2.atan2(a2p).to_degrees();
        if h < 0.0 { h + 360.0 } else { h }
    };

    let dl = l2 - l1;

    let dc = c2p - c1p;

    let dh_p = if c1p * c2p == 0.0 {
        0.0
    } else if (h2p - h1p).abs() <= 180.0 {
        h2p - h1p
    } else if h2p - h1p > 180.0 {
        h2p - h1p - 360.0
    } else {
        h2p - h1p + 360.0
    };

    let dh = 2.0 * (c1p * c2p).sqrt() * (dh_p.to_radians() / 2.0).sin();

    let l_bar = (l1 + l2) / 2.0;
    let c_bar_p = (c1p + c2p) / 2.0;

    let h_bar_p = if c1p * c2p == 0.0 {
        h1p + h2p
    } else if (h1p - h2p).abs() <= 180.0 {
        (h1p + h2p) / 2.0
    } else if h1p + h2p < 360.0 {
        (h1p + h2p + 360.0) / 2.0
    } else {
        (h1p + h2p - 360.0) / 2.0
    };

    let t = 1.0
        - 0.17 * (h_bar_p - 30.0).to_radians().cos()
        + 0.24 * (2.0 * h_bar_p).to_radians().cos()
        + 0.32 * (3.0 * h_bar_p + 6.0).to_radians().cos()
        - 0.20 * (4.0 * h_bar_p - 63.0).to_radians().cos();

    let sl = 1.0 + 0.015 * (l_bar - 50.0).powi(2) / (20.0 + (l_bar - 50.0).powi(2)).sqrt();
    let sc = 1.0 + 0.045 * c_bar_p;
    let sh = 1.0 + 0.015 * c_bar_p * t;

    let c_bar_p7 = c_bar_p.powi(7);
    let rc = 2.0 * (c_bar_p7 / (c_bar_p7 + 25_f64.powi(7))).sqrt();
    let d_theta = 60.0 * (-((h_bar_p - 275.0) / 25.0).powi(2)).exp();
    let rt = -rc * d_theta.to_radians().sin();

    (
        (dl / sl).powi(2)
        + (dc / sc).powi(2)
        + (dh / sh).powi(2)
        + rt * (dc / sc) * (dh / sh)
    ).sqrt()
}

fn parse_color_hex(color: &Color) -> Option<(u8, u8, u8)> {
    let raw = color.info().value.trim().trim_start_matches('#');
    let hex = if raw.len() >= 6 { &raw[raw.len() - 6..] } else { return None; };
    let cr = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let cg = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let cb = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((cr, cg, cb))
}

fn get_ldr_color(available_colors: &[Color], pixel: (u8, u8, u8)) -> Color {
    let (r, g, b) = pixel;
    let (pl, pa, pb) = rgb_to_lab(r, g, b);

    available_colors
        .iter()
        .min_by(|ca, cb| {
            let delta = |color: &Color| -> f64 {
                match parse_color_hex(color) {
                    Some((cr, cg, cb)) => {
                        let (cl, cal, cbl) = rgb_to_lab(cr, cg, cb);
                        ciede2000(pl, pa, pb, cl, cal, cbl)
                    }
                    None => {
                        eprintln!("Failed to parse color: {:?}", color.info().value);
                        f64::MAX
                    }
                }
            };
            delta(ca).partial_cmp(&delta(cb)).unwrap()
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
