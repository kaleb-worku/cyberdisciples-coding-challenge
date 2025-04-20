/*
Kaleb Worku
Stevens Creek Church
Cyber Disciples Small Group
Meeting on Discord
19APR2025
*/

// Generated code in the Rust Programming Language with assistance from Perplexity AI
// Attempting to spot errors in the code with the help of Andrew G. and Will N.

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load background image
    let img_path = "background.jpg";
    let mut img = image::open(img_path).unwrap();

    // Load font (include font bytes directly in binary)
    let font_data: &[u8] = include_bytes!("arial.ttf");
    let font = Font::try_from_bytes(font_data).ok_or("Failed to load font")?;

    // Static scripture text
    let scripture = "that at the name of Jesus every knee should bow,\n\
                    in heaven and on earth and under the earth,\n\
                    and every tongue acknowledge that Jesus Christ is Lord\n\
                    - Philippians 2:10-11 (NIV)";

    // Font configuration
    let scale = Scale { x: 80.0, y: 80.0 };
    let color = Rgba([255u8, 255u8, 255u8, 255u8]);

    // Calculate total text height
    let line_heights: Vec<i32> = scripture
        .lines()
        .map(|line| text_size(scale, &font, line).1 as i32)
        .collect();
    let total_height: i32 = line_heights.iter().sum();

    // Get image dimensions
    let (width, height) = img.dimensions();
    let mut y = ((height as i32 - total_height) / 2) as i32;

    // Draw text with white outline
    for line in scripture.lines() {
        let (text_width, _) = text_size(scale, &font, line);
        let x = ((width as i32 - text_width as i32) / 2) as u32;

        // Draw outline
        for offset in &[(-1, -1), (1, -1), (-1, 1), (1, 1)] {
            draw_text_mut(
                &mut img,
                Rgba([0u8, 0u8, 0u8, 255u8]),
                (x as i32 + offset.0),
                (y as i32 + offset.1),
                scale,
                &font,
                line,
            );
        }

        // Draw main text
        draw_text_mut(&mut img, color, x as i32, y as i32, scale, &font, line);

        y += line_heights[scripture.lines().position(|x| x == line).unwrap()];
    }

    // Save and display (platform-dependent)
    let output_path = "output_with_verse.png";
    img.save(output_path)?;
    println!("Image saved to {}", output_path);

    // For display, we can't directly show like Python's Image.show()
    // Consider using platform-specific commands or a GUI crate
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", output_path])
            .status()?;
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open")
            .arg(output_path)
            .status()?;
    } else {
        std::process::Command::new("xdg-open")
            .arg(output_path)
            .status()?;
    }

    Ok(())
}
