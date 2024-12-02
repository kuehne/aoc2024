pub fn calculate(input: &str) -> i64 {
    -1
}

pub fn calculate_variant(input: &str) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/03.txt").expect("Test file not found.")));
        assert_eq!(result, 99);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/03.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 99);
    }
}
