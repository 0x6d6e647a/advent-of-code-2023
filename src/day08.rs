use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

fn gcd(a: usize, b: usize) -> usize {
    let mut m = a;
    let mut n = b;

    if m == 0 || n == 0 {
        return m | n;
    }

    let shift = (m | n).trailing_zeros();
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }

    m << shift
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid direction char: '{}'", c),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn get(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(Debug)]
struct Network {
    directions: Vec<Direction>,
    map: HashMap<String, Node>,
}

const START_NODE_NAME: &str = "AAA";
const FINAL_NODE_NAME: &str = "ZZZ";

impl Network {
    fn walk_distance(&self) -> usize {
        let mut distance = 0;
        let mut directions = self.directions.iter().cycle();
        let mut curr = self.map.get(START_NODE_NAME).unwrap();

        while curr.name != FINAL_NODE_NAME {
            distance += 1;

            let direction = directions.next().unwrap();
            let next_name = curr.get(direction);
            curr = self.map.get(next_name).unwrap();
        }

        distance
    }

    fn walk_distance_single(&self, start: &Node) -> usize {
        let mut distance = 0;
        let mut directions = self.directions.iter().cycle();
        let mut curr = start;

        while !curr.name.ends_with('Z') {
            distance += 1;

            let direction = directions.next().unwrap();
            let next_name = curr.get(direction);
            curr = self.map.get(next_name).unwrap();
        }

        distance
    }

    fn walk_parallel(&self) -> usize {
        self.map
            .values()
            .filter(|node| node.name.ends_with('A'))
            .map(|node| self.walk_distance_single(node))
            .fold(1, lcm)
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Network {
    let mut lines = input.lines();

    // -- Parse directions.
    let directions = lines.next().unwrap().chars().map(|c| c.into()).collect();
    lines.next();

    // -- Parse nodes.
    let mut map = HashMap::new();

    for line in lines {
        let line = line.replace(['=', '(', ',', ')'], "");
        let mut components = line.split_whitespace();

        let name = components.next().unwrap().to_string();
        let left = components.next().unwrap().to_string();
        let right = components.next().unwrap().to_string();

        let key = name.clone();
        let node = Node { name, left, right };

        map.insert(key, node);
    }

    Network { directions, map }
}

#[aoc(day8, part1)]
fn part1(network: &Network) -> usize {
    network.walk_distance()
}

#[aoc(day8, part2)]
fn part2(network: &Network) -> usize {
    network.walk_parallel()
}
