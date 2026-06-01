// Haryanto 01 June 2026
// Build script to compile Windows resources

use std::path::Path;

fn main() { 
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let png_path = Path::new("icon.png");
        let ico_path = Path::new("target/temp_icon.ico");

        if png_path.exists() {
            let img = image::open(png_path).expect("Failed to open PNG icon");
            
            // Resize the image to 256x256 which is the maximum allowed size for ICO
            // FilterType::Lanczos3 provides the best quality for downscaling
            let resized_img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
            
            // Create the parent directory for the temporary ICO if it doesn't exist
            if let Some(parent) = ico_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            
            // Save the resized image as ICO
            resized_img.save(ico_path).expect("Failed to save converted ICO file");
        } else {
            panic!("Error: icon.png not found in the root directory!");
        }

        // Apply the generated ICO file using winres
        let mut res = winres::WindowsResource::new();
        res.set_icon(ico_path.to_str().unwrap());
        res.compile().unwrap();
    }
}