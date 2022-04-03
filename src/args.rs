#[derive(Debug)]
pub struct Arguments {
    pub first_image: String,
    pub second_image: String,
    pub output: String,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            first_image: String::new(),
            second_image: String::new(),
            output: String::new(),
        }
    }
}
