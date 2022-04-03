use crate::helpers::getter_args::get_nth_arg;

const FIRST_ELEMENT: usize = 1;
const SECOND_ELEMENT: usize = 2;
const THIRD_ELEMENT: usize = 3;

#[derive(Debug)]
pub struct Arguments {
    pub first_image_path: String,
    pub second_image_path: String,
    pub output_path: String,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            first_image_path: get_nth_arg(FIRST_ELEMENT),
            second_image_path: get_nth_arg(SECOND_ELEMENT),
            output_path: get_nth_arg(THIRD_ELEMENT),
        }
    }
}
