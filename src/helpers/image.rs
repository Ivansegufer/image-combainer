use image::{io::Reader, DynamicImage, ImageFormat};

pub fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = Reader::open(path).expect("Can't find image");
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}
