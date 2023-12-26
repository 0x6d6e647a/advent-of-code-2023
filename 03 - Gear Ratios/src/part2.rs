use std::io::{stdin, BufRead, Stdin};

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

struct InclusiveRange {
    begin: usize,
    end: usize,
}

impl InclusiveRange {
    fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    fn contains(&self, value: usize) -> bool {
        value >= self.begin && value <= self.end
    }
}

struct NumberContainer {
    row_index: usize,
    col_range: InclusiveRange,
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
        let col_range = InclusiveRange::new(col_start, col_end);
        NumberContainer {
            row_index,
            col_range,
            digits,
            value,
        }
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
            let num_col_range = InclusiveRange::new(
                number.col_range.begin.saturating_sub(1),
                number.col_range.end + 1,
            );

            if !num_col_range.contains(self.col_index) {
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
    numbers: Vec<NumberContainer>,
    gears: Vec<Gear>,
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
            numbers,
            gears,
        }
    }
}

impl Schematic {
    fn sum_of_gear_ratios(&self) -> u32 {
        self.gears.iter().filter_map(|g| g.get_ratio(self)).sum()
    }
}

fn main() {
    let schematic = Schematic::from(&stdin());
    let sum = schematic.sum_of_gear_ratios();
    println!("{sum}");
}
