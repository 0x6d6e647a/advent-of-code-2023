use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};

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

enum Direction {
    Left,
    Right,
}

use Direction::*;

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Left,
            'R' => Right,
            _ => panic!("invalid direction char: '{}'", ch),
        }
    }
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn get(&self, direction: &Direction) -> &str {
        match direction {
            Left => &self.left,
            Right => &self.right,
        }
    }
}

struct Network {
    directions: Vec<Direction>,
    map: HashMap<String, Node>,
}

impl From<&Stdin> for Network {
    fn from(stdin: &Stdin) -> Self {
        let mut lines = stdin.lock().lines();

        // -- Parse directions.
        let directions = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|c| c.into())
            .collect();
        lines.next();

        // -- Parse nodes.
        let mut map = HashMap::new();

        while let Some(Ok(line)) = lines.next() {
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
}

impl Network {
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

fn main() {
    let solution = Network::from(&stdin()).walk_parallel();
    println!("{solution}");
}
