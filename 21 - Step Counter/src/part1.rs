use std::collections::VecDeque;
use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Rock,
    Garden,
}

use TileType::*;

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '.' | 'S' => Garden,
            '#' => Rock,
            _ => panic!("invalid char for tile type: '{}'", value),
        }
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
struct Coord2D {
    col: isize,
    row: isize,
}

impl Coord2D {
    fn new(col: isize, row: isize) -> Self {
        Self { col, row }
    }

    fn ucol(&self) -> usize {
        self.col.try_into().unwrap()
    }

    fn urow(&self) -> usize {
        self.row.try_into().unwrap()
    }
}

// -----------------------------------------------------------------------------
struct Board {
    grid: Vec<Vec<TileType>>,
    ncols: usize,
    nrows: usize,
    start: Coord2D,
}

impl From<&Stdin> for Board {
    fn from(stdin: &Stdin) -> Self {
        let mut grid = Vec::new();
        let mut start = None;

        for (row_index, line) in stdin.lock().lines().enumerate() {
            let line = line.unwrap();
            let mut row = Vec::new();

            for (col_index, ch) in line.char_indices() {
                if ch == 'S' {
                    let col = col_index.try_into().unwrap();
                    let row = row_index.try_into().unwrap();
                    start = Some(Coord2D::new(col, row));
                }

                row.push(ch.into());
            }

            grid.push(row);
        }

        let nrows = grid.len();
        let ncols = grid[0].len();
        let start = start.unwrap();

        Self {
            grid,
            ncols,
            nrows,
            start,
        }
    }
}

impl Board {
    fn is_inbounds(&self, coord: &Coord2D) -> bool {
        let ncols = self.ncols.try_into().unwrap();
        let nrows = self.nrows.try_into().unwrap();

        coord.col >= 0 && coord.col < ncols && coord.row >= 0 && coord.row < nrows
    }

    fn get(&self, coord: &Coord2D) -> Option<TileType> {
        if self.is_inbounds(coord) {
            return Some(self.grid[coord.urow()][coord.ucol()]);
        }

        None
    }

    fn go_direction(&self, coord: &Coord2D, direction: &Direction) -> Option<Coord2D> {
        let mut new_coord = coord.clone();

        match direction {
            North => new_coord.row -= 1,
            South => new_coord.row += 1,
            East => new_coord.col += 1,
            West => new_coord.col -= 1,
        }

        if self.is_inbounds(&new_coord) && self.get(&new_coord).unwrap() != Rock {
            return Some(new_coord);
        }

        None
    }

    fn fill(&self, limit: usize) -> Vec<Coord2D> {
        let mut queue = VecDeque::from([self.start.clone()]);

        for _ in 0..limit {
            let mut next_queue = VecDeque::new();

            while let Some(curr) = queue.pop_front() {
                for direction in &[North, East, South, West] {
                    if let Some(next) = self.go_direction(&curr, direction) {
                        if next_queue.contains(&next) {
                            continue;
                        }

                        next_queue.push_back(next);
                    }
                }
            }

            queue = next_queue;
        }

        queue.into()
    }
}

// -----------------------------------------------------------------------------
fn main() {
    let board = Board::from(&stdin());
    let plots = board.fill(64);
    println!("{}", plots.len());
}
