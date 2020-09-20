
use std::fs;
use std::path::Path;

use image::ImageFormat;
use image::{GenericImageView, Rgba};

fn get_top_left(im: &image::DynamicImage) -> u32 {
    for x in 0..(im.dimensions().1) {
        for y in 0..(im.dimensions().0) {
            let col = im.get_pixel(y, x);
            if col[0] != 255 && col[1] != 255 && col[2] != 255 {
                return x;
            }
        }
    }
    unreachable!();
}

fn get_top_right(im: &image::DynamicImage) -> u32 {
    for x in 0..(im.dimensions().0) {
        for y in 0..(im.dimensions().1) {
            let col = im.get_pixel(x, y);
            if col[0] != 255 && col[1] != 255 && col[2] != 255 {
                return x;
            }
        }
    }
    unreachable!();
}

fn get_lower_left(im: &image::DynamicImage) -> u32 {
    let mut x = im.dimensions().1 as i32 - 1;
    // Using while loop as there is no reliable way
    // to use custom steps in range() currently
    while x >= 0 {
        let mut y = im.dimensions().0 as i32 - 1;
        while y >= 0 {
            let col = im.get_pixel(y as u32, x as u32);
            if col[0] != 255 && col[1] != 255 && col[2] != 255 {
                return x as u32 + 1;
            }
            y -= 1;
        }
        x -= 1;
    }
    unreachable!();
}

fn get_lower_right(im: &image::DynamicImage) -> u32 {
    let mut x = im.dimensions().0 as i32 - 1;
    // Using while loop as there is no reliable way
    // to use custom steps in range() currently
    while x >= 0 {
        let mut y = im.dimensions().1 as i32 - 1;
        while y >= 0 {
            let col = im.get_pixel(x as u32, y as u32);
            if col[0] != 255 && col[1] != 255 && col[2] != 255 {
                return x as u32 + 1;
            }
            y -= 1;
        }
        x -= 1;
    }
    unreachable!();
}

pub fn crop_image(input_path: &str, output_path: &str) {
    // Load image:
    let mut image = image::open(&Path::new(input_path)).unwrap();
    // Top left corner
    let (b, a) = (get_top_left(&image), get_top_right(&image));
    // Lower right corner
    let (y, x) = (get_lower_left(&image), get_lower_right(&image));

    println!(
        "Cropping area ({0}, {1}, {2}, {3}) from {4} to {5}",
        a, b, x, y, input_path, output_path
    );

    let subim = image.crop(a, b, x - a, y - b);

    // let fout = fs::File::create(&Path::new(output_path)).unwrap();
    // let ref mut fout = std::io::BufWriter::new(fout);

    let _ = subim
        .save_with_format(&output_path, ImageFormat::Jpeg)
        .unwrap();
}
