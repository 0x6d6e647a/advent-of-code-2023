use std::collections::hash_map::DefaultHasher;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Neg;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn manhattan_distance(&self, other: &Coord2D) -> usize {
        let distance = (self.col - other.col).abs() + (self.row - other.row).abs();
        distance.try_into().unwrap()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Node {
    heat_loss: usize,
    heuristic: usize,
    coord: Coord2D,
    direction: Option<Direction>,
    num_steps: u8,
}

impl Node {
    fn seen_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.coord.hash(&mut hasher);
        self.direction.hash(&mut hasher);
        self.num_steps.hash(&mut hasher);
        hasher.finish()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const RADIX: u32 = 10;

struct Board {
    grid: Vec<Vec<u8>>,
    ncols: usize,
    nrows: usize,
    goal: Coord2D,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let mut grid = Vec::new();

        for line in value.lines() {
            let mut row = Vec::new();

            for ch in line.chars() {
                let value = ch.to_digit(RADIX).unwrap().try_into().unwrap();
                row.push(value);
            }

            grid.push(row);
        }

        let nrows = grid.len();
        let ncols = grid[0].len();

        let goal_col = ncols.checked_sub(1).unwrap().try_into().unwrap();
        let goal_row = nrows.checked_sub(1).unwrap().try_into().unwrap();
        let goal = Coord2D::new(goal_col, goal_row);

        Self {
            grid,
            ncols,
            nrows,
            goal,
        }
    }
}

impl Board {
    fn is_inbounds(&self, coord: &Coord2D) -> bool {
        let ncols = self.ncols.try_into().unwrap();
        let nrows = self.nrows.try_into().unwrap();

        coord.col >= 0 && coord.col < ncols && coord.row >= 0 && coord.row < nrows
    }

    fn go_direction(&self, coord: &Coord2D, direction: &Direction) -> Option<Coord2D> {
        let mut new_coord = *coord;

        match direction {
            North => new_coord.row -= 1,
            South => new_coord.row += 1,
            East => new_coord.col += 1,
            West => new_coord.col -= 1,
        }

        if self.is_inbounds(&new_coord) {
            return Some(new_coord);
        }

        None
    }

    fn get(&self, coord: &Coord2D) -> Option<u8> {
        if self.is_inbounds(coord) {
            return Some(self.grid[coord.urow()][coord.ucol()]);
        }

        None
    }

    fn add_next_node(
        &self,
        node: &Node,
        direction: &Direction,
        queue: &mut BinaryHeap<Node>,
        reset: bool,
    ) {
        if let Some(next_coord) = self.go_direction(&node.coord, direction) {
            let mut next = *node;
            next.direction = Some(*direction);
            next.coord = next_coord;
            next.heat_loss += usize::from(self.get(&next.coord).unwrap());
            next.heuristic = next.heat_loss + next.coord.manhattan_distance(&self.goal);
            if reset {
                next.num_steps = 1;
            } else {
                next.num_steps += 1;
            }
            queue.push(next);
        }
    }

    fn least_heat_loss_part1(&self) -> usize {
        const DIRECTION_MAX: u8 = 3;

        let mut seen: HashSet<u64> = HashSet::new();
        let mut queue = BinaryHeap::from([Node::default()]);

        while let Some(node) = queue.pop() {
            // -- Check if found goal.
            if node.coord == self.goal {
                return node.heat_loss;
            }

            // -- Skip already seen.
            let node_hash = node.seen_hash();

            if seen.contains(&node_hash) {
                continue;
            }

            seen.insert(node_hash);

            // -- Generate children.
            if let Some(node_direction) = node.direction {
                if node.num_steps < DIRECTION_MAX {
                    self.add_next_node(&node, &node_direction, &mut queue, false);
                }
            }

            for direction in DIRECTIONS {
                if let Some(node_direction) = node.direction {
                    if node_direction == direction || node_direction == -direction {
                        continue;
                    }
                }

                self.add_next_node(&node, &direction, &mut queue, true);
            }
        }

        panic!("search failed");
    }

    fn least_heat_loss_part2(&self) -> usize {
        const DIRECTION_MIN: u8 = 4;
        const DIRECTION_MAX: u8 = 10;

        let mut seen: HashSet<u64> = HashSet::new();
        let mut queue = BinaryHeap::from([Node::default()]);

        while let Some(node) = queue.pop() {
            // -- Check if found goal.
            if node.coord == self.goal && node.num_steps >= DIRECTION_MIN {
                return node.heat_loss;
            }

            // -- Skip already seen.
            let node_hash = node.seen_hash();

            if seen.contains(&node_hash) {
                continue;
            }

            seen.insert(node_hash);

            // -- Generate children.
            if let Some(node_direction) = node.direction {
                if node.num_steps < DIRECTION_MAX {
                    self.add_next_node(&node, &node_direction, &mut queue, false);
                }
            }

            if node.num_steps >= 4 || node.direction.is_none() {
                for direction in DIRECTIONS {
                    if let Some(node_direction) = node.direction {
                        if node_direction == direction || node_direction == -direction {
                            continue;
                        }
                    }

                    self.add_next_node(&node, &direction, &mut queue, true);
                }
            }
        }

        panic!("search failed");
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Board {
    input.into()
}

#[aoc(day17, part1)]
fn part1(board: &Board) -> usize {
    board.least_heat_loss_part1()
}

#[aoc(day17, part2)]
fn part2(board: &Board) -> usize {
    board.least_heat_loss_part2()
}
