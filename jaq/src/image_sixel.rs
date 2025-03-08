use jaq_json::Val;
use base64::{engine::general_purpose, Engine as _};

pub fn is_image(value: &Val) -> bool {
    // Check if the value is a string
    if let Val::Str(s) = value {
        // Attempt to decode base64 using the new API
        if let Ok(decoded) = general_purpose::STANDARD.decode(s.as_bytes()) {
            // Check if the data is a PNG by matching the magic bytes
            // PNG magic number: 89 50 4E 47 0D 0A 1A 0A
            let png_magic = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
            if decoded.starts_with(&png_magic) {
                return true;
            }
            // TODO: implement checks for JPG
            // TODO: implement checks for WEBP
            // TODO: implement checks for other image formats
        }
    }

    false
}

pub fn print_image_with_sixel(_value: &Val) {
    println!("# SIXEL TO BE IMPLEMENTED");
}
