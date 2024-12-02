pub fn calculate(input: &str) -> i64 {
    input.lines().map(score_line).sum()
}

pub fn calculate_variant(input: &str) -> i64 {
    input.lines().map(score_line_variant).sum()
}

fn score_line(line: &str) -> i64 {
    let items = line
        .split_whitespace()
        .map(|i| i.parse::<i64>().expect("Not parseable"))
        .collect::<Vec<i64>>();
    let mut prev = items.get(0).unwrap();
    let mut direction = None;
    for value in items[1..].iter() {
        if direction.is_none() {
            direction = Some(prev - value);
        }
        let diff = prev - value;
        if is_invalid(diff, direction.unwrap()) {
            return 0;
        }
        prev = value;
    }
    1
}

fn score_line_variant(line: &str) -> i64 {
    let items = line
        .split_whitespace()
        .map(|i| i.parse::<i64>().expect("Not parseable"))
        .collect::<Vec<i64>>();
    let mut prev = items.get(0).unwrap();
    let mut direction = None;
    for (i, value) in items[1..].iter().enumerate() {
        if direction.is_none() {
            direction = Some(prev - value);
        }
        let diff = prev - value;
        if is_invalid(diff, direction.unwrap()) {
            return score_alternative_lines(&items, i);
        }
        prev = value;
    }
    1
}

fn score_alternative_lines(items: &Vec<i64>, i: usize) -> i64 {
    let first_alternative = score_line(&construct_alternative_line(items, i));
    let second_alternative = score_line(&construct_alternative_line(items, i + 1));
    let mut third_alternative = 0;
    if i > 0 {
        let third_clone = construct_alternative_line(items, i - 1);
        third_alternative = score_line(&third_clone);
    }
    *vec![first_alternative, second_alternative, third_alternative]
        .iter()
        .max()
        .expect("Empty vector")
}

fn construct_alternative_line(items: &Vec<i64>, index: usize) -> String {
    items
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, v)| v.to_string())
        .reduce(|cur: String, nxt: String| cur + " " + &nxt)
        .unwrap()
}

fn is_invalid(diff: i64, direction: i64) -> bool {
    diff.abs() > 3 || diff * direction <= 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/02.txt").expect("Test file not found.")));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/02.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 7);
    }
}
