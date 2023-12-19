use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            3 => North,
            0 => East,
            1 => South,
            2 => West,
            _ => panic!("invalid value for direction: '{}'", value),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => North,
            'R' => East,
            'D' => South,
            'L' => West,
            _ => panic!("invalid character for direction: '{}'", value),
        }
    }
}

struct DigCmd {
    direction: Direction,
    distance: isize,
}

impl DigCmd {
    fn new_part1(line: &str) -> Self {
        let mut components = line.split_whitespace();
        let direction = components
            .next()
            .unwrap()
            .trim()
            .chars()
            .next()
            .unwrap()
            .into();
        let distance = components.next().unwrap().parse().unwrap();
        Self {
            direction,
            distance,
        }
    }

    fn new_part2(line: &str) -> Self {
        let hex_str = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .replace(['(', '#', ')'], "");
        let distance = hex_str.chars().take(5).collect::<String>();
        let distance = isize::from_str_radix(&distance, 16).unwrap();
        let direction = hex_str.chars().nth(5).unwrap();
        let direction: u8 = direction.to_digit(16).unwrap().try_into().unwrap();
        let direction = direction.into();
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord2D {
    col: isize,
    row: isize,
}

impl Coord2D {
    fn new(col: isize, row: isize) -> Self {
        Self { col, row }
    }

    fn go_direction(&self, direction: &Direction, distance: usize) -> Self {
        let mut new_coord = *self;
        let distance: isize = distance.try_into().unwrap();

        match direction {
            North => new_coord.row -= distance,
            East => new_coord.col += distance,
            South => new_coord.row += distance,
            West => new_coord.col -= distance,
        }

        new_coord
    }
}

fn process_dig_commands(dig_cmds: &[DigCmd]) -> (Vec<Coord2D>, isize) {
    let mut total_len = 0;
    let mut curr_coord = Coord2D::new(0, 0);
    let mut points = vec![curr_coord];

    for cmd in dig_cmds {
        let distance = cmd.distance.try_into().unwrap();
        total_len += distance;
        curr_coord = curr_coord.go_direction(&cmd.direction, distance);
        points.push(curr_coord);
    }

    (points, total_len.try_into().unwrap())
}

fn area(dig_cmds: &[DigCmd]) -> isize {
    let (points, total_len) = process_dig_commands(dig_cmds);

    // -- Shoelace formula.
    let mut sum = 0;

    for i in 1..points.len() {
        let a = points[i].col;
        let b = points[(i + 1) % points.len()].row;
        let c = points[i - 1].row;
        sum += a * (b - c)
    }

    // -- Pick's theorem.
    let a = sum.abs() / 2;
    let b = total_len;
    let i = a - (b / 2) + 1;
    i + b
}

#[aoc_generator(day18)]
fn parse(input: &str) -> (Vec<DigCmd>, Vec<DigCmd>) {
    let mut part1 = Vec::new();
    let mut part2 = Vec::new();

    for line in input.lines() {
        part1.push(DigCmd::new_part1(line));
        part2.push(DigCmd::new_part2(line));
    }

    (part1, part2)
}

#[aoc(day18, part1)]
fn part1((dig_cmds, _): &(Vec<DigCmd>, Vec<DigCmd>)) -> isize {
    area(dig_cmds)
}

#[aoc(day18, part2)]
fn part2((_, dig_cmds): &(Vec<DigCmd>, Vec<DigCmd>)) -> isize {
    area(dig_cmds)
}
