use image::{imageops::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat};

use crate::enums::image::ImageDataErrors;

pub fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    match Reader::open::<&String>(&path) {
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() {
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataErrors::UnableToDecodeImage(e)),
                }
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        }
        Err(e) => Err(ImageDataErrors::UnableToReadImageFromPath(e)),
    }
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

pub fn standarize_size(
    image_1: DynamicImage,
    image_2: DynamicImage,
) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

pub fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let &val = match vec.get(i) {
            Some(d) => d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }
    rgba
}
