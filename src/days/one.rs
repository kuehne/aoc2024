use std::collections::{HashMap};

pub fn calculate(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut res = 0;
    for (l, r) in input.lines().map(parse_line) {
        left.push(l);
        right.push(r);
    }
    left.sort_by(|a, b| b.cmp(a));
    right.sort_by(|a, b| b.cmp(a));
    for (i,l) in left.iter().enumerate() {
        res += (l - right[i]).abs();
    }
    res
}

pub fn calculate_variant(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right: HashMap<i64, i64> = HashMap::new();
    let mut res = 0;
    for (l, r) in input.lines().map(parse_line) {
        left.push(l);
        let value = right.get(&r).unwrap_or(&0) +1;
        right.insert(r, value);
    }
    for l in left{
        res += l * right.get(&l).unwrap_or(&0);
    }
    res
}

fn parse_line(line: &str) -> (i64, i64) {
    let values = line.split_ascii_whitespace().map(|x| x.parse::<i64>().expect("Not parseable")).collect::<Vec<_>>();
    (values[0], values[1])
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    #[test]
    fn test_calculate() {
        let result = calculate(&(read_to_string("../../resources/inputs/01.txt").expect("Test file not found.")));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(&(read_to_string("../../resources/inputs/01.txt").expect("Test file not found.")));
        assert_eq!(result, 31);
    }
}