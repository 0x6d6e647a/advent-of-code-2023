use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};

// -- Direction
#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

const DIRECTONS: [Direction; 4] = [North, South, East, West];

// -- Coordinate
#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    col: usize,
    row: usize,
}

impl Coordinate {
    fn new(col: usize, row: usize) -> Self {
        Coordinate { col, row }
    }

    fn get_direction(&self, direction: &Direction) -> Option<Coordinate> {
        let mut col = Some(self.col);
        let mut row = Some(self.row);

        match direction {
            North => row = self.row.checked_sub(1),
            South => row = self.row.checked_add(1),
            East => col = self.col.checked_add(1),
            West => col = self.col.checked_sub(1),
        }

        if let (Some(col), Some(row)) = (col, row) {
            return Some(Coordinate::new(col, row));
        }

        None
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

// -- Tile Type
#[derive(PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    BendNorthToEast,
    BendNorthToWest,
    BendSouthToWest,
    BendSouthToEast,
    Ground,
    Start,
}

use TileType::*;

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Vertical => "│",        // "|",
            Horizontal => "─",      // "-",
            BendNorthToEast => "└", // "L",
            BendNorthToWest => "┘", // "J",
            BendSouthToWest => "┐", // "7",
            BendSouthToEast => "┌", // "F",
            Ground => "·",          // ".",
            Start => "S",
        };
        write!(f, "{}", c)
    }
}

impl TileType {
    fn new(c: char) -> Self {
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => BendNorthToEast,
            'J' => BendNorthToWest,
            '7' => BendSouthToWest,
            'F' => BendSouthToEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!("unknown tile type character: '{}'", c),
        }
    }

    fn get_next_directions(&self) -> Vec<Direction> {
        match self {
            Vertical => vec![North, South],
            Horizontal => vec![East, West],
            BendNorthToEast => vec![North, East],
            BendNorthToWest => vec![North, West],
            BendSouthToWest => vec![South, West],
            BendSouthToEast => vec![South, East],
            _ => panic!("invalid tile type for next directions: '{}'", self),
        }
    }
}

// -- Board
struct Board {
    grid: Vec<Vec<TileType>>,
    start: Coordinate,
    ncol: usize,
    nrow: usize,
}

impl Board {
    fn new(input: &str) -> Self {
        // -- Parse input into grid and find starting point.
        let mut start = None;
        let mut grid = Vec::new();

        for (row_index, line) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (col_index, c) in line.char_indices() {
                let tile = TileType::new(c);

                if tile == Start {
                    start = Some(Coordinate::new(col_index, row_index));
                }
                row.push(tile);
            }

            grid.push(row);
        }

        let start = start.unwrap();
        let ncol = grid[0].len();
        let nrow = grid.len();
        let mut board = Board {
            grid,
            start,
            ncol,
            nrow,
        };

        // -- Determine start node neighbors.
        let start = board.start.clone();
        let mut neighbor_map = HashMap::new();

        for direction in DIRECTONS {
            if let Some(dir_coor) = board.get_direction(&start, &direction) {
                if let Some(dir_tile) = board.get(&dir_coor) {
                    if match direction {
                        North => matches!(dir_tile, Vertical | BendSouthToEast | BendSouthToWest),
                        East => matches!(dir_tile, Horizontal | BendNorthToWest | BendSouthToWest),
                        South => matches!(dir_tile, Vertical | BendNorthToEast | BendNorthToWest),
                        West => matches!(dir_tile, Horizontal | BendNorthToEast | BendSouthToEast),
                    } {
                        neighbor_map.insert(direction, dir_tile);
                    }
                }
            }
        }

        *board.get_mut(&start).unwrap() = match (
            neighbor_map.get(&North),
            neighbor_map.get(&East),
            neighbor_map.get(&South),
            neighbor_map.get(&West),
        ) {
            (Some(_), Some(_), None, None) => BendNorthToEast,
            (Some(_), None, None, Some(_)) => BendNorthToWest,
            (None, None, Some(_), Some(_)) => BendSouthToWest,
            (None, Some(_), Some(_), None) => BendSouthToEast,
            _ => panic!("invalid stating tile type conversion!"),
        };

        // -- Result result.
        board
    }

    fn len(&self) -> usize {
        self.ncol.checked_mul(self.nrow).unwrap()
    }

    fn get(&self, coord: &Coordinate) -> Option<&TileType> {
        self.grid.get(coord.row)?.get(coord.col)
    }

    fn get_mut(&mut self, coord: &Coordinate) -> Option<&mut TileType> {
        self.grid.get_mut(coord.row)?.get_mut(coord.col)
    }

    fn get_direction(&self, coord: &Coordinate, direction: &Direction) -> Option<Coordinate> {
        let coord = coord.get_direction(direction)?;

        if coord.col >= self.ncol || coord.row >= self.nrow {
            return None;
        }

        Some(coord)
    }

    fn find_loop_coords(&self) -> HashSet<Coordinate> {
        let mut loop_coords = HashSet::new();
        let mut queue = VecDeque::from([self.start.clone()]);

        while let Some(curr_coord) = queue.pop_front() {
            for direction in self
                .get(&curr_coord)
                .unwrap()
                .get_next_directions()
                .into_iter()
            {
                let next_coord = self.get_direction(&curr_coord, &direction).unwrap();

                if !loop_coords.contains(&next_coord) {
                    loop_coords.insert(next_coord.clone());
                    queue.push_back(next_coord);
                }
            }
        }

        loop_coords
    }

    fn find_outside_coords(&self, loop_coords: &HashSet<Coordinate>) -> HashSet<Coordinate> {
        let mut outside_coords = HashSet::new();

        for (row_index, row) in self.grid.iter().enumerate() {
            let mut inside = false;
            let mut orientation = None;

            for (col_index, tile) in row.iter().enumerate() {
                let curr_coord = Coordinate::new(col_index, row_index);
                let mut is_loop_coord = false;

                if loop_coords.contains(&curr_coord) {
                    is_loop_coord = true;

                    match tile {
                        Vertical => {
                            if orientation.is_some() {
                                panic!("invalid state @ {}", curr_coord);
                            }
                            inside ^= true;
                        }
                        Horizontal => {
                            if orientation.is_none() {
                                panic!("invalid state @ {}", curr_coord);
                            }
                        }
                        BendNorthToEast | BendSouthToEast => {
                            if orientation.is_some() {
                                panic!("invalid state @ {}", curr_coord);
                            }
                            orientation = Some(match tile {
                                BendNorthToEast => North,
                                BendSouthToEast => South,
                                _ => panic!("invalid state @ {}", curr_coord),
                            });
                        }
                        BendSouthToWest | BendNorthToWest => {
                            if *tile
                                != match orientation {
                                    Some(North) => BendNorthToWest,
                                    Some(South) => BendSouthToWest,
                                    _ => panic!("invalid state @ {}", curr_coord),
                                }
                            {
                                inside ^= true;
                            }
                            orientation = None;
                        }
                        _ => panic!("invalid state @ {}", curr_coord),
                    };
                }

                if !inside && !is_loop_coord {
                    outside_coords.insert(curr_coord);
                }
            }
        }

        outside_coords
    }

    #[allow(dead_code)]
    fn print(&self, loop_coords: &HashSet<Coordinate>, outside_coords: &HashSet<Coordinate>) {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, tile) in row.iter().enumerate() {
                let coord = Coordinate::new(col_index, row_index);

                let mut prefix = "";
                let mut suffix = "";

                if coord == self.start {
                    prefix = "\x1B[01;32m"; // green
                } else if loop_coords.contains(&coord) {
                    prefix = "\x1B[01;31m"; // red
                } else if outside_coords.contains(&coord) {
                    prefix = "\x1B[01;33m"; // brown
                }

                if !prefix.is_empty() {
                    suffix = "\x1B[0;00m";
                }

                print!("{}{}{}", prefix, tile, suffix);
            }

            println!();
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Board {
    Board::new(input)
}

#[aoc(day10, part1)]
fn part1(board: &Board) -> usize {
    board.find_loop_coords().len() / 2
}

#[aoc(day10, part2)]
fn part2(board: &Board) -> usize {
    let loop_coords = board.find_loop_coords();
    let outside_coords = board.find_outside_coords(&loop_coords);
    // board.print(&loop_coords, &outside_coords);
    board.len() - loop_coords.len() - outside_coords.len()
}
