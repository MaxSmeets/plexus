use iced::Color;

pub fn hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        panic!("Invalid hex code: {}", hex);
    }

    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid red value");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid green value");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid blue value");

    Color::from_rgb8(r, g, b)
}
