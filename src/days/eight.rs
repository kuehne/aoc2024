use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Hash, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

enum Direction {
    Desc,
    Asc,
}

impl Coordinate {
    fn from(x: isize, y: isize) -> Coordinate {
        Coordinate { x, y }
    }

    fn find_antinodes(&self, other: &Coordinate, len: isize) -> HashSet<Coordinate> {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        let min_x = min(self.x, other.x);
        let max_x = max(self.x, other.x);
        let min_y = min(self.y, other.y);
        let max_y = max(self.y, other.y);
        let direction =
            if min_x == self.x && min_y == self.y || min_x == other.x && min_y == other.y {
                Direction::Asc
            } else {
                Direction::Desc
            };
        let mut res = HashSet::new();
        match direction {
            Direction::Asc => {
                for i in 0..len {
                    res.insert(Coordinate::from(
                        min_x - (1 + i) * x_diff,
                        min_y - (1 + i) * y_diff,
                    ));
                    res.insert(Coordinate::from(
                        max_x + (1 + i) * x_diff,
                        max_y + (1 + i) * y_diff,
                    ));
                }
            }
            Direction::Desc => {
                for i in 0..len {
                    res.insert(Coordinate::from(
                        max_x + (1 + i) * x_diff,
                        min_y - (1 + i) * y_diff,
                    ));
                    res.insert(Coordinate::from(
                        min_x - (1 + i) * x_diff,
                        max_y + (1 + i) * y_diff,
                    ));
                }
            }
        }
        res
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

pub fn calculate(input: &str) -> i64 {
    find_valid_antinodes(input, 1)
}

fn find_valid_antinodes(input: &str, depth: isize) -> i64 {
    let (has_map, x_len, y_len) = parse(input);
    let mut antinodes = HashSet::new();
    for (_, elements) in has_map {
        for element in elements.iter() {
            for el2 in elements.iter() {
                if element == el2 && depth == 1 {
                    continue;
                }
                antinodes.extend(element.find_antinodes(el2, depth));
            }
        }
    }
    let valid_antinodes = antinodes
        .iter()
        .filter(|a| is_valid_coordinate(a, x_len, y_len))
        .collect::<Vec<_>>();
    valid_antinodes.len() as i64
}

fn is_valid_coordinate(c: &Coordinate, x_len: isize, y_len: isize) -> bool {
    c.x >= 0 && c.y >= 0 && c.x < x_len && c.y < y_len
}

fn parse(input: &str) -> (HashMap<char, HashSet<Coordinate>>, isize, isize) {
    let mut res = HashMap::new();
    let x_len = input.lines().count() as isize;
    let mut y_len = 0isize;
    for (y, line) in input.lines().enumerate() {
        y_len = line.chars().count() as isize;
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = Coordinate::from(x as isize, y as isize);
            let set = res.entry(c).or_insert_with(HashSet::new);
            set.insert(coord);
        }
    }
    (res, x_len, y_len)
}

pub fn calculate_variant(input: &str) -> i64 {
    // TODO: Optimize
    find_valid_antinodes(input, input.lines().count() as isize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/08.txt").expect("Test file not found.")));
        assert_eq!(result, 14);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/08.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 34);
    }
}
