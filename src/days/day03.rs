// Day 3: Gear Ratios

use std::collections::HashSet;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

pub struct Day3;

fn grid() -> Result<Grid> {
    Grid::from_str(include_str!("./day03_input.txt"))
}

impl Day for Day3 {
    fn day_number(&self) -> u8 {
        3
    }

    fn part1(&self) -> Option<Result<String>> {
        let grid = match grid() {
            Result::Ok(grid) => grid,
            Err(e) => return Some(Err(e)),
        };
        let numbers = grid.find_numbers_adjacent_to_symbols();
        let sum = numbers.iter().sum::<u32>();
        Some(Ok(sum.to_string()))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    fn origin() -> Self {
        Coordinate::new(0, 0)
    }

    fn adjacent_coordinates(&self) -> Vec<Coordinate> {
        vec![
            Coordinate::new(self.x - 1, self.y - 1),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x + 1, self.y - 1),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridCell {
    Digit(u8),
    Symbol(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    width: usize,
    cells: Vec<Option<GridCell>>,
}

impl Grid {
    fn get(&self, coord: Coordinate) -> Option<GridCell> {
        let index = self.index(coord);
        self.cells.get(index).copied().flatten()
    }

    fn index(&self, coord: Coordinate) -> usize {
        (coord.y * self.width as i32 + coord.x) as usize
    }

    fn coordinate_for_index(&self, index: usize) -> Coordinate {
        let x = index % self.width;
        let y = index / self.width;
        Coordinate::new(x as i32, y as i32)
    }

    fn find_all_symbols(&self) -> Vec<(Coordinate, char)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| {
                let coordinate = self.coordinate_for_index(index);
                match cell {
                    Some(GridCell::Symbol(c)) => Some((coordinate, *c)),
                    _ => None,
                }
            })
            .collect()
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    fn find_all_numbers(&self) -> Vec<Number> {
        let mut result = Vec::new();
        for y in 0..self.height() {
            type CurrentNumber = (Vec<u8>, Coordinate);
            fn current_number_to_number(current_number: &CurrentNumber) -> Number {
                let (digits, left) = current_number;
                Number {
                    value: digits
                        .iter()
                        .fold(0, |tens_place, digit| tens_place * 10 + *digit as u32),
                    left: *left,
                    width: digits.len() as u8,
                }
            }
            let mut current_number: Option<CurrentNumber> = None;
            for x in 0..self.width {
                let coordinate = Coordinate::new(x as i32, y as i32);
                let cell = self.get(coordinate);
                match &mut current_number {
                    Some(current_number_found) => match cell {
                        Some(GridCell::Digit(digit)) => {
                            let (digits, _) = current_number_found;
                            digits.push(digit);
                        }
                        _ => {
                            result.push(current_number_to_number(current_number_found));
                            current_number = None;
                        }
                    },
                    None => match cell {
                        Some(GridCell::Digit(digit)) => {
                            current_number = Some((vec![digit], coordinate))
                        }
                        _ => {}
                    },
                }
            }
            if let Some(current_number_found) = &mut current_number {
                result.push(current_number_to_number(current_number_found));
            }
        }
        result
    }

    fn find_numbers_adjacent_to_symbols(&self) -> Vec<u32> {
        let numbers = self.find_all_numbers();
        numbers
            .into_iter()
            .filter(|it| {
                let adjacent_coordinates = it.adjacent_coordinates();
                adjacent_coordinates
                    .iter()
                    .any(|coord| match self.get(*coord) {
                        Some(GridCell::Symbol(_)) => true,
                        _ => false,
                    })
                // let all_adjacent_coordinates =
            })
            .map(|it| it.value)
            .collect()
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect::<Vec<_>>();
        let width = lines.first().ok_or(anyhow!("empty grid"))?.chars().count();
        let chars: Vec<char> = lines
            .iter()
            .map(|line| {
                let chars = line.chars().collect::<Vec<char>>();
                if chars.len() != width {
                    return Err(anyhow!(
                        "inconsistent line width - expected {}, got {} ({})",
                        width,
                        chars.len(),
                        line
                    ));
                }
                Ok(chars)
            })
            .collect::<Result<Vec<Vec<char>>>>()?
            .into_iter()
            .flatten()
            .collect();
        let grid: Vec<Option<GridCell>> = chars
            .into_iter()
            .map(|c| match c {
                '.' => None,
                '0'..='9' => Some(GridCell::Digit(c.to_digit(10).unwrap() as u8)),
                c => Some(GridCell::Symbol(c)),
            })
            .collect();
        Ok(Grid { cells: grid, width })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Number {
    value: u32,
    left: Coordinate,
    width: u8,
}

impl Number {
    fn digit_coordinates(&self) -> Vec<Coordinate> {
        (0..self.width)
            .map(|x| Coordinate::new(self.left.x + x as i32, self.left.y))
            .collect()
    }

    fn adjacent_coordinates(&self) -> Vec<Coordinate> {
        let digit_coordinates = self.digit_coordinates();
        let adjacent_coordinates_set: HashSet<Coordinate> = digit_coordinates
            .iter()
            .flat_map(|coord| coord.adjacent_coordinates())
            .collect();
        adjacent_coordinates_set
            .difference(&digit_coordinates.iter().copied().collect())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_grid() -> Grid {
        Grid::from_str(indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "})
        .unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!("509115".to_string(), super::Day3.part1().unwrap().unwrap());
    }

    #[test]
    fn test_grid_parse() {
        let grid = sample_grid();
        assert_eq!(grid.get(Coordinate::origin()), Some(GridCell::Digit(4)));
        assert_eq!(grid.get(Coordinate::new(3, 1)), Some(GridCell::Symbol('*')));
    }

    #[test]
    fn test_grid_find_all_symbols() {
        let grid = sample_grid();
        let symbols = grid.find_all_symbols();
        assert_eq!(
            symbols,
            vec![
                (Coordinate { x: 3, y: 1 }, '*'),
                (Coordinate { x: 6, y: 3 }, '#'),
                (Coordinate { x: 3, y: 4 }, '*'),
                (Coordinate { x: 5, y: 5 }, '+'),
                (Coordinate { x: 3, y: 8 }, '$'),
                (Coordinate { x: 5, y: 8 }, '*')
            ]
        );
    }

    #[test]
    fn test_grid_find_all_numbers() {
        let grid = sample_grid();
        let numbers = grid.find_all_numbers();
        assert_eq!(
            numbers.iter().map(|it| it.value).collect::<Vec<u32>>(),
            vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]
        );
    }

    #[test]
    fn test_grid_find_numbers_adjacent_to_symbols() {
        let grid = sample_grid();
        let numbers = grid.find_numbers_adjacent_to_symbols();
        let sum = numbers.iter().sum::<u32>();
        assert_eq!(sum, 4361);
        let missing_numbers = grid.find_all_numbers().len() - numbers.len();
        assert_eq!(missing_numbers, 2);
    }
}
