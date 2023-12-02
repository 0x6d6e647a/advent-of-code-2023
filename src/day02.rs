use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;

const TOTAL_RED: u32 = 12;
const TOTAL_BLUE: u32 = 14;
const TOTAL_GREEN: u32 = 13;

struct GameRound {
    red: u32,
    blue: u32,
    green: u32,
}

impl From<&str> for GameRound {
    fn from(line: &str) -> Self {
        let mut n = 0;
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for entry in line.split(", ").flat_map(|s| s.split(' ')) {
            if entry.chars().all(char::is_numeric) {
                n = entry.parse().unwrap();
            } else {
                match entry {
                    "red" => red += n,
                    "blue" => blue += n,
                    "green" => green += n,
                    _ => panic!("unknown color: '{}'", entry),
                }
            }
        }

        GameRound { red, blue, green }
    }
}

impl GameRound {
    fn invalid(&self) -> bool {
        self.red > TOTAL_RED || self.blue > TOTAL_BLUE || self.green > TOTAL_GREEN
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

struct Game {
    index: u32,
    rounds: Vec<GameRound>,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let mut rounds = Vec::new();

        let line = line.split(": ").nth(1).unwrap();

        for subline in line.split("; ") {
            rounds.push(GameRound::from(subline));
        }

        let index = index as u32 + 1;
        games.push(Game { index, rounds });
    }

    games
}

#[aoc(day2, part1)]
fn part1(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut good = true;

        for round in &game.rounds {
            if round.invalid() {
                good = false;
                break;
            }
        }

        if good {
            sum += game.index + 1;
        }
    }

    sum
}

#[aoc(day2, part2)]
fn part2(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut fewest = GameRound {
            red: 0,
            blue: 0,
            green: 0,
        };

        for round in &game.rounds {
            fewest.red = max(fewest.red, round.red);
            fewest.blue = max(fewest.blue, round.blue);
            fewest.green = max(fewest.green, round.green);
        }

        sum += fewest.power();
    }

    sum
}
