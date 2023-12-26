use std::cmp::max;
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
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

struct Game {
    rounds: Vec<GameRound>,
}

fn parse_games() -> Vec<Game> {
    let mut games = Vec::new();

    let mut lines = stdin().lock().lines();

    while let Some(Ok(line)) = lines.next() {
        let mut rounds = Vec::new();

        let line = line.split(": ").nth(1).unwrap();

        for subline in line.split("; ") {
            rounds.push(GameRound::from(subline));
        }

        games.push(Game { rounds });
    }

    games
}

fn main() {
    let mut sum = 0;

    for game in parse_games() {
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

    println!("{sum}");
}
