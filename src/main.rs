use std::path::Path;

use image::{ImageBuffer, Rgb, Rgba, RgbaImage};
use rand::Rng;

type Jpg = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Png = ImageBuffer<Rgba<u8>, Vec<u8>>;

const DIR_CATCH: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn jpg_to_png<P>(file_path: P) -> Png
where
    P: AsRef<Path>,
{
    let binding = image::open(file_path).unwrap();
    let jpg: &Jpg = binding.as_rgb8().unwrap();
    let (width, height) = jpg.dimensions();

    ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = jpg.get_pixel(x, y);

        image::Rgba([pixel[0], pixel[1], pixel[2], 255])
    })
}

fn encode_rotate(img: &mut Png, seed: usize) {
    let (width, height) = img.dimensions();
    let mut new_img: RgbaImage = ImageBuffer::new(width, height);
    let mut border = 0;
    let mut dir = 0;
    let mut pos: (i32, i32) = (0, 0);
    for pixel in new_img.pixels_mut() {
        if dir == 0 && pos.0 == width as i32 - 1 - border
            || dir == 1 && pos.1 == height as i32 - 1 - border
            || dir == 2 && pos.0 == border
            || dir == 3 && pos.1 == border
        {
            dir = (dir + 1) % 4;

            if dir == 3 {
                border += 1;
            }
        }

        *pixel = *img.get_pixel(pos.0 as u32, pos.1 as u32);

        pos.0 += DIR_CATCH[dir].0;
        pos.1 += DIR_CATCH[dir].1;
    }

    new_img.save(format!("image/{seed}.png")).unwrap();
}

fn encode(img: &mut Png) {
    let mut rng = rand::thread_rng();
    let carry_count: usize = rng.gen_range(2..=9);
    let offset = rng.gen_range(10..=40);
    let mut check_sum: usize = 0;

    let mut count = 0;
    for (index, color) in img.iter_mut().enumerate() {
        let index = index % 256;

        if index != 0 && index % offset == 0 {
            count += carry_count;
        }

        *color = (*color as usize + (index * index + count) % 256) as u8;
        check_sum += *color as usize % 2;
    }

    let seed = ((check_sum - (offset + carry_count)) * 1000 + offset * 10 + carry_count) << 1;
    encode_rotate(img, seed);
    println!("seed: {seed}");
}

fn main() {
    let dirs = std::fs::read_dir("./image").unwrap();

    for dir in dirs {
        let file_path = dir.unwrap().path();
        if let Some(extension) = file_path.extension() {
            let extension = extension.to_str().unwrap();
            let mut img: Png;
            if extension == "jpg" {
                img = jpg_to_png(&file_path);
            } else if extension == "png" {
                let binding = image::open(file_path).unwrap();
                img = binding.as_rgba8().unwrap().clone();
            } else {
                continue;
            }

            encode(&mut img);
        }
    }
}
