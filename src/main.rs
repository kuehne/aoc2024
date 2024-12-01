mod days;

use std::fs;
use crate::days::one;

fn main() {
    let file_contents = fs::read_to_string("resources/inputs/01.txt").expect("File not found");
    println!("Result: {}", one::calculate_variant(&file_contents));
}
