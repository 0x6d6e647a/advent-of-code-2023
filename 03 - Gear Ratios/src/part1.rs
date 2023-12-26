use std::io::{stdin, Stdin, BufRead};

const RADIX: u32 = 10;

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
}

struct NumberContainer {
    digits: Vec<DigitContainer>,
    value: u32,
}

impl NumberContainer {
    fn new(
        digits: Vec<DigitContainer>,
    ) -> Self {
        let string = String::from_iter(digits.iter().map(|d| d.digit as char));
        let value = string
            .parse()
            .unwrap_or_else(|err| panic!("digits did not form a number '{}': {}", string, err));
        NumberContainer {
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

struct Schematic {
    rows: Vec<Vec<u8>>,
    numbers: Vec<NumberContainer>,
}

impl From<&Stdin> for Schematic {
    fn from(stdin: &Stdin) -> Self {
        // -- Construct the grid.
        let rows: Vec<_> = stdin
            .lock()
            .lines()
            .map(|l| l.unwrap().bytes().collect())
            .map(|bs: Vec<u8>| bs.to_owned())
            .collect();

        // -- Find all numbers.
        let mut numbers = Vec::new();

        for (row_index, row) in rows.iter().enumerate() {
            let mut digits = Box::<Vec<DigitContainer>>::default();

            for (col_index, curr) in row.iter().enumerate() {
                // -- Found digit.
                if (*curr as char).is_digit(RADIX) {
                    // -- Collect the digit.
                    let digit = *curr;
                    digits.push(DigitContainer {
                        digit,
                        row_index,
                        col_index,
                    });
                    continue;
                }

                if !digits.is_empty() {
                    numbers.push(NumberContainer::new(*digits));
                    digits = Box::<Vec<DigitContainer>>::default();
                }
            }

            // -- Handle remaining digits.
            if !digits.is_empty() {
                numbers.push(NumberContainer::new(*digits));
            }
        }

        Schematic {
            rows,
            numbers,
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
}

fn main() {
    let schematic = Schematic::from(&stdin());
    let sum = schematic.sum_of_part_numbers();
    println!("{sum}");
}
