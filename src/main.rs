mod days;

use crate::days::{eight, five, four, one, seven, six, three, two};
use std::{env, fs};

const DEFAULT_PATH: &str = "08";
const VARIANT_IS_DEFAULT: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_day = String::from(DEFAULT_PATH);
    let day = args.get(1).unwrap_or(&default_day);
    let input_path = &format!("resources/inputs/{day}.txt");
    let contents = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let is_variant = VARIANT_IS_DEFAULT || args.contains(&String::from("--variant"));
    let result = calculate(day, &contents, is_variant);
    println!("Result: {}", result);
}

fn calculate(day: &String, contents: &str, is_variant: bool) -> i64 {
    match day.as_str() {
        "01" => match is_variant {
            false => one::calculate(&contents),
            true => one::calculate_variant(&contents),
        },
        "02" => match is_variant {
            false => two::calculate(&contents),
            true => two::calculate_variant(&contents),
        },
        "03" => match is_variant {
            false => three::calculate(&contents),
            true => three::calculate_variant(&contents),
        },
        "04" => match is_variant {
            false => four::calculate(&contents),
            true => four::calculate_variant(&contents),
        },
        "05" => match is_variant {
            false => five::calculate(&contents),
            true => five::calculate_variant(&contents),
        },
        "06" => match is_variant {
            false => six::calculate(&contents),
            true => six::calculate_variant(&contents),
        },
        "07" => match is_variant {
            false => seven::calculate(&contents),
            true => seven::calculate_variant(&contents),
        },
        "08" => match is_variant {
            false => eight::calculate(&contents),
            true => eight::calculate_variant(&contents),
        },
        _ => panic!("Day {} is not yet implemented", day),
    }
}
