use crate::enums::image::ImageDataErrors;

pub struct FloatingImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

impl FloatingImage {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity: usize = (height * width * 4).try_into().unwrap();
        FloatingImage {
            width,
            height,
            data: Vec::with_capacity(buffer_capacity),
            name,
        }
    }
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}
