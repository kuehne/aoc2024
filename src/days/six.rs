use std::collections::HashSet;

pub fn calculate(input: &str) -> i64 {
    let mut contents = Six::parse(input);
    contents.traverse()
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

#[derive(Debug)]
struct Six {
    values: Vec<Vec<char>>,
    direction: usize,
    position: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

impl Six {
    fn from(values: Vec<Vec<char>>, direction: usize, position: (isize, isize)) -> Six {
        Six {
            values,
            direction,
            position,
            visited: HashSet::new(),
        }
    }

    fn parse(input: &str) -> Six {
        let mut values = vec![];
        let mut direction = 0;
        let mut position = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut line_values = vec![];
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        position = (x as isize, y as isize);
                        direction = RIGHT
                    }
                    '<' => {
                        position = (x as isize, y as isize);
                        direction = LEFT
                    }
                    '^' => {
                        position = (x as isize, y as isize);
                        direction = UP
                    }
                    'v' => {
                        position = (x as isize, y as isize);
                        direction = DOWN
                    }
                    _ => (),
                }
                line_values.push(c);
            }
            values.push(line_values);
        }
        Six::from(values, direction, position)
    }

    fn traverse(&mut self) -> i64 {
        self.walk(self.position, self.direction)
    }

    fn walk(&mut self, position: (isize, isize), direction: usize) -> i64 {
        let new_position;
        let new_direction;
        if is_out_of_bound(position, &self.values) {
            return self.visited.len() as i64;
        }
        let value = self.values[position.1 as usize][position.0 as usize];

        match value {
            '#' => {
                (new_position, new_direction) =
                    calculate_position_and_direction(position, direction);
            }
            _ => {
                self.visited.insert(position);
                new_position = calculate_position(position, direction);
                new_direction = direction;
            }
        }
        self.walk(new_position, new_direction)
    }
}
#[derive(Debug)]
struct SixVariant {
    values: Vec<Vec<char>>,
    route_positions: HashSet<(isize, isize)>,
    direction: usize,
    position: (isize, isize),
}
#[derive(Debug)]
struct Route {
    obstacle_position: (isize, isize),
    position: (isize, isize),
    direction: usize,
    visited: HashSet<((isize, isize), usize)>,
}

impl Route {
    fn from(six_variant: &SixVariant, position: (isize, isize)) -> Route {
        Route {
            position: six_variant.position,
            obstacle_position: position,
            direction: six_variant.direction,
            visited: HashSet::new(),
        }
    }
}

impl SixVariant {
    fn from_six(six: Six) -> SixVariant {
        let mut positions = six.visited.clone();
        positions.remove(&six.position);
        SixVariant {
            values: six.values.clone(),
            route_positions: positions,
            direction: six.direction,
            position: six.position,
        }
    }

    fn find_valid_obstacle_positions(&self) -> i64 {
        let mut loop_positions = HashSet::new();
        for position in self.route_positions.iter() {
            let mut route = Route::from(self, *position);
            if self.is_loop(&mut route) {
                loop_positions.insert(position);
            }
        }
        loop_positions.len() as i64
    }

    fn is_loop(&self, route: &mut Route) -> bool {
        if is_out_of_bound(route.position, &self.values) {
            return false;
        }
        let position_and_direction = (route.position, route.direction);
        if route.visited.contains(&position_and_direction) {
            return true;
        }
        route.visited.insert(position_and_direction);
        let value = self.values[route.position.1 as usize][route.position.0 as usize];
        if value == '#' || route.position == route.obstacle_position {
            let (new_position, new_direction) =
                calculate_position_and_direction(route.position, route.direction);
            route.position = new_position;
            route.direction = new_direction;
        } else {
            route.position = calculate_position(route.position, route.direction);
        }
        self.is_loop(route)
    }
}

fn calculate_position(position: (isize, isize), direction: usize) -> (isize, isize) {
    (
        position.0 + DIRECTIONS[direction].0,
        position.1 + DIRECTIONS[direction].1,
    )
}

fn calculate_position_and_direction(
    position: (isize, isize),
    direction: usize,
) -> ((isize, isize), usize) {
    let new_direction = (direction + 1) % 4;
    let new_position = (
        position.0 - DIRECTIONS[direction].0 + DIRECTIONS[new_direction].0,
        position.1 - DIRECTIONS[direction].1 + DIRECTIONS[new_direction].1,
    );
    (new_position, new_direction)
}

fn is_out_of_bound(position: (isize, isize), values: &Vec<Vec<char>>) -> bool {
    let check = position.0 < 0
        || position.0 >= values.len() as isize
        || position.1 < 0
        || position.1 >= values[0].len() as isize;
    check
}

pub fn calculate_variant(input: &str) -> i64 {
    let mut six = Six::parse(input);
    six.traverse();
    let variant = SixVariant::from_six(six);
    variant.find_valid_obstacle_positions()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_calculate() {
        let result =
            calculate(&(read_to_string("resources/tests/06.txt").expect("Test file not found.")));
        assert_eq!(result, 41);
    }

    #[test]
    fn test_calculate_variant() {
        let result = calculate_variant(
            &(read_to_string("resources/tests/06.txt").expect("Test file not found.")),
        );
        assert_eq!(result, 6);
    }
}
