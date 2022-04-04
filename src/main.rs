mod enums;
mod helpers;
use enums::image::ImageDataErrors;
use helpers::args::Arguments;
use helpers::floating_image::FloatingImage;
use helpers::image::{combine_images, find_image_from_path, standarize_size};
use image::GenericImageView;

fn main() -> Result<(), ImageDataErrors> {
    let args: Arguments;
    args = Arguments::new();
    let (image_1, image_format_1) = find_image_from_path(args.first_image_path);
    let (image_2, image_format_2) = find_image_from_path(args.second_image_path);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standarize_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_2.height(), args.output_path);

    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?;
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format_1,
    )
    .unwrap();
    return Ok(());
}
