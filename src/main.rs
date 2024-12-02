mod days;

use std::{env, fs};
use crate::days::{one, two};

const DEFAULT_PATH: &str = "resources/inputs/02.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = match args.get(1) {
        Some(t) => &format!("resources/inputs/{t}.txt"),
        None => DEFAULT_PATH,
    };
    let contents = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let is_variant = args.contains(&String::from("--variant"));
    let result = match args.get(1) {
        Some(day) => {
            match day.as_str() {
                "01" => match is_variant {
                    false => one::calculate(&contents),
                    true => one::calculate_variant(&contents),
                },
                "02" => match is_variant {
                    false => two::calculate(&contents),
                    true => two::calculate_variant(&contents),
                },
                _ => 0
            }
        },
        None => {
            two::calculate_variant(&contents)
        }
    };
    println!("Result: {}", result);
}
