use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

const DIRECTONS: [Direction; 4] = [North, South, East, West];

// -----------------------------------------------------------------------------
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

// -----------------------------------------------------------------------------
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

impl From<char> for TileType {
    fn from(ch: char) -> Self {
        match ch {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => BendNorthToEast,
            'J' => BendNorthToWest,
            '7' => BendSouthToWest,
            'F' => BendSouthToEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!("unknown tile type character: '{}'", ch),
        }
    }
}

impl TileType {
    fn get_next_directions(&self) -> Vec<Direction> {
        match self {
            Vertical => vec![North, South],
            Horizontal => vec![East, West],
            BendNorthToEast => vec![North, East],
            BendNorthToWest => vec![North, West],
            BendSouthToWest => vec![South, West],
            BendSouthToEast => vec![South, East],
            _ => panic!("invalid tile type for next directions"),
        }
    }
}

// -----------------------------------------------------------------------------
struct Board {
    grid: Vec<Vec<TileType>>,
    start: Coordinate,
    ncol: usize,
    nrow: usize,
}

impl From<&Stdin> for Board {
    fn from(stdin: &Stdin) -> Self {
        // -- Parse input into grid and find starting point.
        let mut start = None;
        let mut grid = Vec::new();

        for (row_index, line) in stdin.lock().lines().enumerate() {
            let mut row = Vec::new();

            for (col_index, ch) in line.unwrap().char_indices() {
                let tile = ch.into();

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
}

impl Board {
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
}

fn main() {
    let board = Board::from(&stdin());
    let steps = board.find_loop_coords().len() / 2;
    println!("{steps}");
}
