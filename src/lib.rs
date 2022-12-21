use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageResult, Pixel};

const PIXEL_CHAR_ARRAY: [char; 10] = ['W', '@', '#', '8', '&', '*', 'o', ':', '.', ' '];

const TARGET_WIDTH: u32 = 120;

/// Load image from path and reshape the image to TARGET_WIDTH
pub fn load_image(img_path: &str) -> ImageResult<DynamicImage> {
    let img = image::open(img_path)?;
    Ok(resize_image(img, TARGET_WIDTH))
}

pub fn resize_image(img: DynamicImage, target_width: u32) -> DynamicImage {
    let (src_width, src_height) = img.dimensions();
    let target_height = get_target_height(src_width, src_height, TARGET_WIDTH);
    img.resize(target_width, target_height, FilterType::CatmullRom)
}

pub fn print_image(img: DynamicImage) {
    let (width, height) = img.dimensions();
    for i in 0..height {
        for j in 0..width {
            let rgb = img.get_pixel(j, i);
            let rgb = rgb.channels();
            let (red, green, blue) = (rgb[0], rgb[1], rgb[2]);
            print!("{}", PIXEL_CHAR_ARRAY[calculate_index(red, green, blue)]);
        }
        println!();
    }
}

#[inline]
fn calculate_index(r: u8, g: u8, b: u8) -> usize {
    let grayscale = 0.2126 * r as f64 + 0.7152 * g as f64 + 0.0722 * b as f64;
    let index = grayscale / ((255 / PIXEL_CHAR_ARRAY.len()) as f64 + 0.5);
    index.floor() as usize
}

/// Calculate the target height of the reshaped image
#[inline]
fn get_target_height(src_width: u32, src_height: u32, target_width: u32) -> u32 {
    let mut target_height = src_height;
    if target_width < src_width {
        // Image is bigger than target, resize to a smaller size
        target_height =
            (target_height as f64 / (src_width as f64 / target_width as f64)).round() as u32;
    }
    target_height
}
