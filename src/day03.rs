use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

const RADIX: u32 = 10;

#[derive(Debug)]
enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::NW,
    Direction::N,
    Direction::NE,
    Direction::W,
    Direction::E,
    Direction::SW,
    Direction::S,
    Direction::SE,
];

struct DigitContainer {
    digit: u8,
    row_index: usize,
    col_index: usize,
}

impl DigitContainer {
    fn coords_at(&self, direction: &Direction) -> Option<(usize, usize)> {
        use Direction::*;

        let r = self.row_index;
        let c = self.col_index;

        let coords = match direction {
            NW => (r.checked_sub(1)?, c.checked_sub(1)?),
            N => (r.checked_sub(1)?, c),
            NE => (r.checked_sub(1)?, c.checked_add(1)?),
            W => (r, c.checked_sub(1)?),
            E => (r, c.checked_add(1)?),
            SW => (r.checked_add(1)?, c.checked_sub(1)?),
            S => (r.checked_add(1)?, c),
            SE => (r.checked_add(1)?, c.checked_add(1)?),
        };

        Some(coords)
    }

    fn adjacent_to(&self, gear: &Gear) -> bool {
        for direction in DIRECTIONS {
            if let Some((r, c)) = self.coords_at(&direction) {
                if gear.row_index == r && gear.col_index == c {
                    return true;
                }
            }
        }

        false
    }
}

struct NumberContainer {
    row_index: usize,
    col_range: RangeInclusive<usize>,
    digits: Vec<DigitContainer>,
    value: u32,
}

impl NumberContainer {
    fn new(
        row_index: usize,
        col_start: usize,
        col_end: usize,
        digits: Vec<DigitContainer>,
    ) -> Self {
        let string = String::from_iter(digits.iter().map(|d| d.digit as char));
        let value = string
            .parse()
            .unwrap_or_else(|err| panic!("digits did not form a number '{}': {}", string, err));
        let col_range = RangeInclusive::new(col_start, col_end);
        NumberContainer {
            row_index,
            col_range,
            digits,
            value,
        }
    }

    fn is_part_number(&self, schematic: &Schematic) -> bool {
        for digit in &self.digits {
            if schematic.is_valid_digit(digit) {
                return true;
            }
        }

        false
    }
}

struct Gear {
    row_index: usize,
    col_index: usize,
}

impl Gear {
    fn get_ratio(&self, schematic: &Schematic) -> Option<u32> {
        let mut first = None;
        let mut second = None;

        for number in &schematic.numbers {
            // -- Skip numbers too vertically far away.
            let gear_row_range = self.row_index - 1..=self.row_index + 1;

            if !gear_row_range.contains(&number.row_index) {
                continue;
            }

            // -- Skip numbers too horizontally far away.
            let num_col_range = RangeInclusive::new(
                number.col_range.start().checked_sub(1).unwrap_or(0),
                number.col_range.end() + 1,
            );

            if !num_col_range.contains(&self.col_index) {
                continue;
            }

            // -- Look for adjacent digits.
            for digit in &number.digits {
                if digit.adjacent_to(self) {
                    match (&first, &second) {
                        (None, _) => {
                            first = Some(number.value);
                            break;
                        }
                        (_, None) => {
                            second = Some(number.value);
                            break;
                        }
                        _ => return None,
                    }
                }
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            return Some(first * second);
        }

        None
    }
}

struct Schematic {
    rows: Vec<Vec<u8>>,
    numbers: Vec<NumberContainer>,
    gears: Vec<Gear>,
}

impl From<&str> for Schematic {
    fn from(input: &str) -> Self {
        // -- Construct the grid.
        let rows: Vec<_> = input
            .lines()
            .map(|l| l.bytes().collect())
            .map(|bs: Vec<u8>| bs.to_owned())
            .collect();

        // -- Find all numbers.
        let mut numbers = Vec::new();
        let mut gears = Vec::new();
        let mut col_start = 0;

        for (row_index, row) in rows.iter().enumerate() {
            let mut digits = Box::<Vec<DigitContainer>>::default();

            for (col_index, curr) in row.iter().enumerate() {
                // -- Found digit.
                if (*curr as char).is_digit(RADIX) {
                    // -- Record the starting column.
                    if digits.is_empty() {
                        col_start = col_index;
                    }

                    // -- Collect the digit.
                    let digit = *curr;
                    digits.push(DigitContainer {
                        digit,
                        row_index,
                        col_index,
                    });
                    continue;
                }

                // -- Found gear.
                if *curr as char == '*' {
                    gears.push(Gear {
                        row_index,
                        col_index,
                    });
                }

                if !digits.is_empty() {
                    numbers.push(NumberContainer::new(
                        row_index, col_start, col_index, *digits,
                    ));
                    digits = Box::<Vec<DigitContainer>>::default();
                }
            }

            // -- Handle remaining digits.
            if !digits.is_empty() {
                numbers.push(NumberContainer::new(
                    row_index,
                    col_start,
                    row.len() - 1_usize,
                    *digits,
                ));
            }
        }

        Schematic {
            rows,
            numbers,
            gears,
        }
    }
}

impl Schematic {
    fn get(&self, row_index: usize, col_index: usize) -> Option<&u8> {
        self.rows.get(row_index)?.get(col_index)
    }

    fn is_valid_digit(&self, digit: &DigitContainer) -> bool {
        for direction in DIRECTIONS {
            if let Some((r, c)) = digit.coords_at(&direction) {
                if let Some(v) = self.get(r, c) {
                    let v = *v as char;
                    if !v.is_ascii_digit() && v != '.' {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn sum_of_part_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| n.is_part_number(self))
            .map(|n| n.value)
            .sum()
    }

    fn sum_of_gear_ratios(&self) -> u32 {
        self.gears
            .iter()
            .filter_map(|g| g.get_ratio(self))
            .sum()
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    Schematic::from(input)
}

#[aoc(day3, part1)]
fn part1(schematic: &Schematic) -> u32 {
    schematic.sum_of_part_numbers()
}

#[aoc(day3, part2)]
fn part2(schematic: &Schematic) -> u32 {
    schematic.sum_of_gear_ratios()
}
