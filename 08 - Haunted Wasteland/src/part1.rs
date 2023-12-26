use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};

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
    fn walk_distance(&self) -> usize {
        const START_NODE_NAME: &str = "AAA";
        const FINAL_NODE_NAME: &str = "ZZZ";

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
}

fn main() {
    let solution = Network::from(&stdin()).walk_distance();
    println!("{solution}");
}
