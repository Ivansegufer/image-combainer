use std::env::{args, Args};

pub fn get_nth_arg(n: usize) -> String {
    let mut arguments: Args = args();
    let argument = arguments.nth(n);

    match argument {
        None => String::from(""),
        Some(value) => value,
    }
}

pub struct Arguments {
    first_image: String,
    second_image: String,
    output: String,
}
