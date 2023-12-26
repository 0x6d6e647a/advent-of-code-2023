use std::io::{stdin, BufRead};

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
        const TOTAL_RED: u32 = 12;
        const TOTAL_BLUE: u32 = 14;
        const TOTAL_GREEN: u32 = 13;

        self.red > TOTAL_RED || self.blue > TOTAL_BLUE || self.green > TOTAL_GREEN
    }
}

struct Game {
    index: u32,
    rounds: Vec<GameRound>,
}

fn parse_games() -> Vec<Game> {
    let mut games = Vec::new();

    let mut lines = stdin().lock().lines().enumerate();

    while let Some((index, Ok(line))) = lines.next() {
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

fn main() {
    let mut sum = 0;

    for game in parse_games() {
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

    println!("{sum}");
}
