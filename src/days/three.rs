use regex::Regex;

const MULT_RULE: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
const DO_RULE: &str = r"do\(\)";
const DONT_RULE: &str = r"don't\(\)";

pub fn calculate(input: &str) -> i64 {
    let re = Regex::new(MULT_RULE).unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |a, (_, [left, right])| {
            a + left.parse::<i64>().unwrap() * right.parse::<i64>().unwrap()
        })
}

pub fn calculate_variant(input: &str) -> i64 {
    let re = Regex::new(MULT_RULE).unwrap();
    let matches = re.find_iter(input).collect::<Vec<_>>();
    let re_do = Regex::new(DO_RULE).unwrap();
    let mut do_indices = re_do
        .find_iter(input)
        .map(|m| m.start())
        .collect::<Vec<_>>();
    let re_dont = Regex::new(DONT_RULE).unwrap();
    let mut dont_indices = re_dont
        .find_iter(input)
        .map(|m| m.start())
        .collect::<Vec<usize>>();
    dont_indices.sort();
    do_indices.sort();
    let mut res: i64 = 0;
    for found_match in &matches {
        let index = found_match.start();
        let do_index = *do_indices
            .iter()
            .filter(|&i| i < &index)
            .last()
            .unwrap_or(&usize::MIN);
        let dont_index = *dont_indices
            .iter()
            .filter(|&i| i < &index)
            .last()
            .unwrap_or(&usize::MIN);
        if do_index >= dont_index {
            res += calculate(found_match.as_str());
        }
        do_indices.retain(|i| i >= &do_index);
        dont_indices.retain(|i| i >= &dont_index);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/03.txt").expect("Test file not found.")));
        assert_eq!(result, 161);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/03.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 48);
    }
}
