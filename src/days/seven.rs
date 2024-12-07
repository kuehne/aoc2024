use std::collections::VecDeque;
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

pub fn calculate(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|(first, second)| get_equation_value(*first, second))
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn calculate_variant(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|(first, second)| get_equation_value_variant(*first, second))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn parse(input: &str) -> Vec<(i64, VecDeque<i64>)> {
    let mut res = Vec::new();
    for line in input.lines() {
        let tokens = line.split(":").collect::<Vec<_>>();
        let result = tokens[0].parse::<i64>().expect("Not parseable result");
        let position_values = tokens[1]
            .trim()
            .split_whitespace()
            .map(|t| t.parse::<i64>().expect("Not parseable value"))
            .collect::<VecDeque<_>>();
        res.push((result, position_values));
    }
    res
}

fn get_equation_value(result: i64, values: &VecDeque<i64>) -> i64 {
    if is_valid(result, 0, values, Operation::Add)
        || is_valid(result, 0, values, Operation::Multiply)
    {
        return result;
    }
    0
}
fn get_equation_value_variant(result: i64, values: &VecDeque<i64>) -> i64 {
    if is_valid_variant(result, 0, values, Operation::Add)
        || is_valid_variant(result, 0, values, Operation::Multiply)
        || is_valid_variant(result, 0, values, Operation::Concatenate)
    {
        return result;
    }
    0
}

fn is_valid(result: i64, temp_result: i64, values: &VecDeque<i64>, operation: Operation) -> bool {
    if values.len() == 0 {
        return result == temp_result;
    }
    let mut remaining_values = values.clone();
    let first_value = remaining_values.pop_front().unwrap();
    let new_temp_result = match operation {
        Operation::Add => temp_result + first_value,
        Operation::Multiply => temp_result * first_value,
        _ => {
            panic!("Not a valid operation")
        }
    };
    is_valid(
        result,
        new_temp_result,
        &remaining_values,
        Operation::Multiply,
    ) || is_valid(result, new_temp_result, &remaining_values, Operation::Add)
}

fn is_valid_variant(
    result: i64,
    temp_result: i64,
    values: &VecDeque<i64>,
    operation: Operation,
) -> bool {
    if values.len() == 0 {
        return result == temp_result;
    }
    let mut remaining_values = values.clone();
    let first_value = remaining_values.pop_front().unwrap();
    let new_temp_result = match operation {
        Operation::Add => temp_result + first_value,
        Operation::Multiply => temp_result * first_value,
        Operation::Concatenate => {
            let num_digits = first_value.to_string().len() as u32;
            temp_result * 10i64.pow(num_digits) + first_value
        }
    };
    is_valid_variant(
        result,
        new_temp_result,
        &remaining_values,
        Operation::Multiply,
    ) || is_valid_variant(result, new_temp_result, &remaining_values, Operation::Add)
        || is_valid_variant(
            result,
            new_temp_result,
            &remaining_values,
            Operation::Concatenate,
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/07.txt").expect("Test file not found.")));
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/07.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 11387);
    }
}
