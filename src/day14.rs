use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use aoc_runner_derive::aoc;

// -- Direction.
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

const CYCLE_DIRECTONS: [Direction; 4] = [North, West, South, East];

// -- TileType.
#[derive(Clone, PartialEq, Hash)]
enum TileType {
    RoundRock,
    CubeRock,
    Ground,
}

use TileType::*;

impl TileType {
    fn new(ch: char) -> Self {
        match ch {
            'O' => RoundRock,
            '#' => CubeRock,
            '.' => Ground,
            _ => panic!("invalid tile character: '{}'", ch),
        }
    }
}

// -- Grid.
type Grid = Vec<Vec<TileType>>;

fn go_direction(
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
    direction: &Direction,
) -> Option<(usize, usize)> {
    let mut row = Some(row);
    let mut col = Some(col);

    match direction {
        North => row = row.unwrap().checked_sub(1),
        South => row = row.unwrap().checked_add(1),
        East => col = col.unwrap().checked_add(1),
        West => col = col.unwrap().checked_sub(1),
    }

    if let (Some(row), Some(col)) = (row, col) {
        if row < nrows && col < ncols {
            return Some((row, col));
        }
    }

    None
}

fn range_iter(start: usize, stop: usize) -> Box<dyn Iterator<Item = usize>> {
    if start < stop {
        Box::new(start..stop)
    } else {
        Box::new((stop..start).rev())
    }
}

fn do_tilt(grid: &mut Grid, direction: &Direction) {
    let nrows = grid.len();

    for row in match direction {
        North | West | East => range_iter(0, nrows),
        South => range_iter(nrows, 0),
    } {
        let ncols = grid[row].len();

        for col in match direction {
            North | South | West => range_iter(0, ncols),
            East => range_iter(ncols, 0),
        } {
            if grid[row][col] == RoundRock {
                let mut curr_row = row;
                let mut curr_col = col;

                while let Some((new_row, new_col)) =
                    go_direction(curr_row, curr_col, nrows, ncols, direction)
                {
                    if grid[new_row][new_col] == Ground {
                        grid[curr_row][curr_col] = Ground;
                        grid[new_row][new_col] = RoundRock;
                    } else {
                        break;
                    }

                    curr_row = new_row;
                    curr_col = new_col;
                }
            }
        }
    }
}

fn get_load(grid: &Grid) -> usize {
    let mut load = 0;

    for (row, tiles) in grid.iter().enumerate() {
        let nround = tiles.iter().filter(|tile| **tile == RoundRock).count();
        load += nround * (grid.len() - row);
    }

    load
}

fn hash_grid(grid: &Grid) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

const NUM_CYCLES: usize = 1_000_000_000;

fn do_cycles(grid: &mut Grid, ncycles: usize) {
    let mut tracker = Vec::new();

    for cycle in 0..ncycles {
        // -- Do cycle.
        for direction in CYCLE_DIRECTONS {
            do_tilt(grid, &direction);
        }

        // -- Check for pattern.
        let curr_hash = hash_grid(grid);

        if let Some(position) = tracker
            .iter()
            .position(|(grid_hash, _)| *grid_hash == curr_hash)
        {
            let index = (NUM_CYCLES - position) % (cycle - position) + position - 1;
            (_, *grid) = tracker.swap_remove(index);
            return;
        }

        tracker.push((curr_hash, grid.clone()));
    }
}

// -- Advent of Code Day 14
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().map(TileType::new).collect())
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    do_tilt(&mut grid, &North);
    get_load(&grid)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    do_cycles(&mut grid, NUM_CYCLES);
    get_load(&grid)
}
