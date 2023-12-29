use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord2D<T> {
    col: T,
    row: T,
}

impl<T> Coord2D<T> {
    fn new(col: T, row: T) -> Self {
        Self { col, row }
    }
}

// -----------------------------------------------------------------------------
fn lagrange_iterpolation(points: &[Coord2D<usize>]) -> Vec<f64> {
    let points: Vec<Coord2D<f64>> = points
        .iter()
        .map(|coord| {
            // -- Forcing isize to f64 conversion.
            let col = coord.col as f64;
            let row = coord.row as f64;

            if col as usize != coord.col || row as usize != coord.row {
                panic!(
                    "usize to f64 converson error: ({}, {}) != ({}, {})",
                    coord.col, coord.row, col, row
                );
            }

            Coord2D::<f64>::new(col, row)
        })
        .collect();
    let n = points.len();
    let mut polynomial_points = vec![f64::default(); n];

    for i in 0..n {
        let mut product = 1.0;

        for j in 0..n {
            if j == i {
                continue;
            }
            product *= points[i].col - points[j].col;
        }

        product = points[i].row / product;

        let mut term = vec![f64::default(); n];
        term[0] = product;

        for (j, point) in points.iter().enumerate() {
            if j == i {
                continue;
            }

            for k in (1..n).rev() {
                term[k] += term[k - 1];
                term[k - 1] *= -point.col;
            }
        }

        for j in 0..n {
            polynomial_points[j] += term[j]
        }
    }

    polynomial_points
}

fn lagrange_eval(x: usize, polynomial_points: &[f64]) -> f64 {
    // -- Convert usize to f64.
    let x_f64 = x as f64;

    if x_f64 as usize != x {
        panic!("usize to f64 conversion error: {} != {}", x, x_f64);
    }

    let x = x_f64;

    // -- Evaluate the Lagrange polynomial.
    let mut sum = 0.0;

    for (i, point) in polynomial_points.iter().enumerate() {
        // -- Convert i from usize to f64.
        let i_f64 = i as f64;

        if i_f64 as usize != i {
            panic!("usize to f64 conversion error: {} != {}", i, i_f64);
        }

        sum += point * x.powf(i_f64);
    }

    sum
}

// -----------------------------------------------------------------------------
type BoardCoord = Coord2D<isize>;

struct Board {
    grid: Vec<Vec<TileType>>,
    ncols: usize,
    nrows: usize,
    start: BoardCoord,
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
                    start = Some(BoardCoord::new(col, row));
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
    fn get_wrap(&self, coord: &BoardCoord) -> TileType {
        let nrows: isize = self.nrows.try_into().unwrap();
        let row: isize = coord.row.rem_euclid(nrows);
        let row: usize = row.try_into().unwrap();
        let ncols: isize = self.ncols.try_into().unwrap();
        let col: isize = coord.col.rem_euclid(ncols);
        let col: usize = col.try_into().unwrap();
        self.grid[row][col]
    }

    fn go_direction_wrap(&self, coord: &BoardCoord, direction: &Direction) -> Option<BoardCoord> {
        let mut new_coord = *coord;

        match direction {
            North => new_coord.row -= 1,
            South => new_coord.row += 1,
            East => new_coord.col += 1,
            West => new_coord.col -= 1,
        }

        if self.get_wrap(&new_coord) == Rock {
            return None;
        }

        Some(new_coord)
    }

    fn fill(&self, limit: usize) -> usize {
        const NUM_SAMPLES: usize = 3;

        let mut points = Vec::with_capacity(NUM_SAMPLES);
        let mut queue = VecDeque::from([self.start]);
        let mut seen_odd = HashSet::new();
        let mut seen_even = HashSet::new();

        assert_eq!(self.ncols, self.nrows);

        for num_steps in 0..limit {
            // -- Collect approixmation points.
            if num_steps % self.ncols == self.ncols / 2 {
                if num_steps % 2 == 0 {
                    points.push(Coord2D::<usize>::new(num_steps, seen_odd.len()));
                } else {
                    points.push(Coord2D::<usize>::new(num_steps, seen_even.len()));
                }

                if points.len() == NUM_SAMPLES {
                    let poly_points = lagrange_iterpolation(&points);
                    let solution = lagrange_eval(limit, &poly_points);
                    return solution.round() as usize;
                }
            }

            let mut next_queue = VecDeque::new();

            while let Some(curr) = queue.pop_front() {
                for direction in &[North, East, South, West] {
                    if let Some(next) = self.go_direction_wrap(&curr, direction) {
                        if next_queue.contains(&next) {
                            continue;
                        }

                        if num_steps % 2 == 0 {
                            if seen_even.contains(&next) {
                                continue;
                            }
                            seen_even.insert(next);
                        } else {
                            if seen_odd.contains(&next) {
                                continue;
                            }
                            seen_odd.insert(next);
                        }

                        next_queue.push_back(next);
                    }
                }
            }

            queue = next_queue;
        }

        panic!("fill failed");
    }
}

// -----------------------------------------------------------------------------
fn main() {
    let board = Board::from(&stdin());
    let num_plots = board.fill(26501365);
    println!("{}", num_plots);
}
