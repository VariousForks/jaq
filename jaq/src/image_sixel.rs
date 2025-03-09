use jaq_json::Val;
use base64::{engine::general_purpose, Engine as _};
// We need to read from memory and guess the image format to support multiple formats
use std::io::Cursor;
use image::io::Reader as ImageReader;
use viuer::{print, Config};

pub fn is_image(value: &Val) -> bool {
    // We try to decode the base64, then guess the image format.
    // If it succeeds, we consider it an image.
    // TODO: If we need deeper checks or more robust recognition (e.g., specific format checks), add them here.
    if let Val::Str(s) = value {
        if let Ok(decoded) = general_purpose::STANDARD.decode(s.as_bytes()) {
            let cursor = Cursor::new(decoded);
            if let Ok(reader) = ImageReader::new(cursor).with_guessed_format() {
                if reader.decode().is_ok() {
                    return true;
                }
            }
        }
    }
    false
}

/// Prints an image in the terminal using viuer, which tries to detect Sixel, Kitty,
/// iTerm, or fallback modes. Supports multiple formats by using the `image` crate.
/// TODO: consider renaming this function if we support more backends in future or
/// allow user configuration of the printing method.
pub fn print_image_with_sixel(value: &Val) {
    if let Val::Str(s) = value {
        // Attempt to decode from base64
        match general_purpose::STANDARD.decode(s.as_bytes()) {
            Ok(decoded) => {
                let cursor = Cursor::new(decoded);
                // Attempt to read the image from memory
                let reader = match ImageReader::new(cursor).with_guessed_format() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to guess image format: {}", e);
                        return;
                    }
                };
                // Decode the image into an in-memory representation
                let img = match reader.decode() {
                    Ok(i) => i,
                    Err(e) => {
                        eprintln!("Failed to decode image: {}", e);
                        return;
                    }
                };

                // Configure viuer. Setting use_sixel to true tries to display
                // with Sixel if the terminal supports it. viuer can also
                // automatically detect other capabilities like Kitty or iTerm.
                let config = Config {
                    use_sixel: true,
                    ..Default::default()
                };

                // Finally, display the image
                if let Err(e) = print(&img, &config) {
                    eprintln!("Failed to print image: {:?}", e);
                }
            }
            Err(_) => {
                eprintln!("Invalid base64 or non-image data.");
            }
        }
    } else {
        eprintln!("Value is not a string.");
    }
}
