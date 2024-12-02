mod days;

use crate::days::{one, two};
use std::{env, fs};

const DEFAULT_PATH: &str = "02";
const VARIANT_IS_DEFAULT: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_day = String::from(DEFAULT_PATH);
    let day = args.get(1).unwrap_or(&default_day);
    let input_path = &format!("resources/inputs/{day}.txt");
    let contents = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let is_variant = VARIANT_IS_DEFAULT || args.contains(&String::from("--variant"));
    let result = match day.as_str() {
        "01" => match is_variant {
            false => one::calculate(&contents),
            true => one::calculate_variant(&contents),
        },
        "02" => match is_variant {
            false => two::calculate(&contents),
            true => two::calculate_variant(&contents),
        },
        _ => -1,
    };
    println!("Result: {}", result);
}
