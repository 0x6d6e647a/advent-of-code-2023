use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(PartialEq, Eq)]
enum TileType {
    MirrorSlash,
    MirrorBackSlash,
    SplitHorz,
    SplitVert,
    Empty,
}

use TileType::*;

impl From<char> for TileType {
    fn from(ch: char) -> Self {
        match ch {
            '/' => MirrorSlash,
            '\\' => MirrorBackSlash,
            '-' => SplitHorz,
            '|' => SplitVert,
            '.' => Empty,
            _ => panic!("invalid tile character: '{}'", ch),
        }
    }
}

// -----------------------------------------------------------------------------
struct LaserHead {
    coord: Coord2D,
    direction: Direction,
}

impl LaserHead {
    fn new(coord: &Coord2D, direction: &Direction) -> Self {
        let coord = *coord;
        let direction = *direction;
        Self { coord, direction }
    }

    fn update(&mut self, new_coord: &Coord2D) {
        self.coord = *new_coord;
    }

    fn split(&mut self, new_coord: &Coord2D, tile: &TileType) -> Option<LaserHead> {
        let (direction_a, direction_b) = match (tile, &self.direction) {
            (SplitVert, East | West) => (North, South),
            (SplitHorz, North | South) => (East, West),
            (SplitVert, North | South) | (SplitHorz, East | West) => {
                self.update(new_coord);
                return None;
            }
            _ => panic!("invalid tile type for split"),
        };
        self.coord = *new_coord;
        self.direction = direction_a;
        Some(LaserHead::new(new_coord, &direction_b))
    }

    fn reflect(&mut self, new_coord: &Coord2D, tile: &TileType) {
        let new_direction = match (tile, &self.direction) {
            (MirrorSlash, North) => East,
            (MirrorSlash, East) => North,
            (MirrorSlash, South) => West,
            (MirrorSlash, West) => South,
            (MirrorBackSlash, North) => West,
            (MirrorBackSlash, East) => South,
            (MirrorBackSlash, South) => East,
            (MirrorBackSlash, West) => North,
            _ => panic!("invalid tile type for reflect"),
        };
        self.coord = *new_coord;
        self.direction = new_direction;
    }
}

// -----------------------------------------------------------------------------
#[derive(Default)]
struct DirectionsVisted {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl DirectionsVisted {
    fn set_visit(&mut self, direction: &Direction) {
        match direction {
            North => self.north = true,
            East => self.east = true,
            South => self.south = true,
            West => self.west = true,
        }
    }

    fn get_visit(&self, direction: &Direction) -> bool {
        match direction {
            North => self.north,
            East => self.east,
            South => self.south,
            West => self.west,
        }
    }

    fn been_visited(&self) -> bool {
        self.north || self.east || self.south || self.west
    }
}

// -----------------------------------------------------------------------------
struct VisitationGrid {
    grid: Vec<Vec<DirectionsVisted>>,
}

impl VisitationGrid {
    fn new(ncols: usize, nrows: usize) -> Self {
        let mut grid = Vec::new();

        for _ in 0..nrows {
            let mut row = Vec::new();

            for _ in 0..ncols {
                row.push(Default::default());
            }

            grid.push(row);
        }

        Self { grid }
    }

    fn get(&self, coord: &Coord2D, direction: &Direction) -> bool {
        self.grid[coord.urow()][coord.ucol()].get_visit(direction)
    }

    fn set(&mut self, coord: &Coord2D, direction: &Direction) {
        self.grid[coord.urow()][coord.ucol()].set_visit(direction)
    }

    fn count_visited(&self) -> usize {
        let mut total = 0;

        for row in self.grid.iter() {
            for dir_visted in row.iter() {
                if dir_visted.been_visited() {
                    total += 1;
                }
            }
        }

        total
    }
}

// -----------------------------------------------------------------------------
struct Board {
    grid: Vec<Vec<TileType>>,
    ncols: usize,
    nrows: usize,
}

impl From<&Stdin> for Board {
    fn from(stdin: &Stdin) -> Self {
        let mut grid = Vec::new();

        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let mut row = Vec::new();

            for ch in line.chars() {
                row.push(ch.into());
            }

            grid.push(row);
        }

        let nrows = grid.len();
        let ncols = grid[0].len();

        Self { grid, ncols, nrows }
    }
}

impl Board {
    fn get(&self, coord: &Coord2D) -> &TileType {
        &self.grid[coord.urow()][coord.ucol()]
    }

    fn go_direction(&self, coord: &Coord2D, direction: &Direction) -> Option<Coord2D> {
        let mut new_coord = *coord;

        match direction {
            North => new_coord.row -= 1,
            South => new_coord.row += 1,
            East => new_coord.col += 1,
            West => new_coord.col -= 1,
        }

        let ncols = self.ncols.try_into().unwrap();
        let nrows = self.nrows.try_into().unwrap();

        if new_coord.col >= 0
            && new_coord.col < ncols
            && new_coord.row >= 0
            && new_coord.row < nrows
        {
            return Some(new_coord);
        }

        None
    }

    fn fire_laser(&self, start: &Coord2D, direction: &Direction) -> usize {
        let mut laser_heads = vec![LaserHead::new(start, direction)];
        let mut visited = VisitationGrid::new(self.ncols, self.nrows);

        while !laser_heads.is_empty() {
            let mut to_remove = Vec::new();
            let mut new_heads = Vec::new();

            for (pos, laser_head) in laser_heads.iter_mut().enumerate() {
                let new_coord = self.go_direction(&laser_head.coord, &laser_head.direction);

                // -- New coordinate off map, laser dead.
                if new_coord.is_none() {
                    to_remove.push(pos);
                    continue;
                }

                // -- Check if new coord has been visited by this laser head.
                let new_coord = new_coord.unwrap();

                if visited.get(&new_coord, &laser_head.direction) {
                    to_remove.push(pos);
                    continue;
                }

                // -- Log that this coord has been visited.
                visited.set(&new_coord, &laser_head.direction);

                // -- Check action to perform based on new cood's tile type.
                let new_tile = self.get(&new_coord);

                match new_tile {
                    MirrorSlash | MirrorBackSlash => {
                        laser_head.reflect(&new_coord, new_tile);
                    }
                    SplitHorz | SplitVert => {
                        if let Some(split) = laser_head.split(&new_coord, new_tile) {
                            new_heads.push(split);
                        }
                    }
                    Empty => laser_head.update(&new_coord),
                }
            }

            // -- Remove dead lasers.
            for pos in to_remove.into_iter().rev() {
                laser_heads.remove(pos);
            }

            // -- Add new lasers.
            for new_laser_head in new_heads {
                laser_heads.push(new_laser_head);
            }
        }

        visited.count_visited()
    }
}

fn main() {
    let board = Board::from(&stdin());
    let num_tiles = board.fire_laser(&Coord2D::new(-1, 0), &East);
    println!("{}", num_tiles);
}
