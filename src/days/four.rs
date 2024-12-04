use itertools::iproduct;
use std::collections::HashMap;
const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn calculate(input: &str) -> i64 {
    let char_map = compute_char_map(input);
    find_valid_neighbour(0, &char_map)
}

fn find_valid_neighbour(index: isize, char_map: &HashMap<char, Vec<(isize, isize)>>) -> i64 {
    let mut res = 0;
    let values = char_map
        .get(&CHARS[index as usize])
        .expect("No X value found");
    for value in values {
        res += find_valid_neighbors(*value, index + 1, char_map);
    }
    res
}

fn find_valid_neighbors(
    coordinates: (isize, isize),
    index: isize,
    char_map: &HashMap<char, Vec<(isize, isize)>>,
) -> i64 {
    let mut res = 0;
    let possible_offsets = iproduct!([-1, 0, 1].iter(), [-1, 0, 1].iter());
    for (x, y) in possible_offsets {
        if *x == 0 && *y == 0 {
            continue;
        }
        let new_x = coordinates.0 + x;
        let new_y = coordinates.1 + y;
        if char_map
            .get(&CHARS[index as usize])
            .unwrap()
            .contains(&(new_x, new_y))
        {
            res += find_valid_neighbors_direction((new_x, new_y), index + 1, char_map, (*x, *y));
        }
    }
    res
}

fn find_valid_neighbors_direction(
    coordinates: (isize, isize),
    index: isize,
    char_map: &HashMap<char, Vec<(isize, isize)>>,
    direction: (isize, isize),
) -> i64 {
    if index > (CHARS.len() - 1) as isize {
        1
    } else {
        let new_cell = (coordinates.0 + direction.0, coordinates.1 + direction.1);
        if char_map
            .get(&CHARS[index as usize])
            .unwrap()
            .contains(&new_cell)
        {
            find_valid_neighbors_direction(new_cell, index + 1, char_map, direction)
        } else {
            0
        }
    }
}

fn compute_char_map(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut char_map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !CHARS.contains(&c) {
                continue;
            }
            let char_values = char_map.entry(c).or_insert_with(Vec::new);
            char_values.push((x as isize, y as isize));
        }
    }
    char_map
}

pub fn calculate_variant(input: &str) -> i64 {
    let char_map = compute_char_map(input);
    char_map
        .get(&'A')
        .unwrap()
        .iter()
        .filter(|&coordinates| is_cross(coordinates, &char_map))
        .count() as i64
}

fn is_cross(coordinates: &(isize, isize), char_map: &HashMap<char, Vec<(isize, isize)>>) -> bool {
    let top_left = (coordinates.0 - 1, coordinates.1 - 1);
    let bottom_right = (coordinates.0 + 1, coordinates.1 + 1);
    let top_right = (coordinates.0 + 1, coordinates.1 - 1);
    let bottom_left = (coordinates.0 - 1, coordinates.1 + 1);
    is_cross_line(top_left, bottom_right, char_map)
        && is_cross_line(top_right, bottom_left, char_map)
}

fn is_cross_line(
    a: (isize, isize),
    b: (isize, isize),
    char_map: &HashMap<char, Vec<(isize, isize)>>,
) -> bool {
    let s_values = char_map.get(&'S').expect("No S value found");
    let m_values = char_map.get(&'M').expect("No M value found");
    s_values.contains(&a) && m_values.contains(&b) || s_values.contains(&b) && m_values.contains(&a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/04.txt").expect("Test file not found.")));
        assert_eq!(result, 18);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/04.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 9);
    }
}
