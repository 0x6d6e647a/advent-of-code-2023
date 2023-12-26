use std::io::{stdin, BufRead, Stdin};

struct RaceData {
    time: usize,
    distance: usize,
}

impl RaceData {
    #[allow(dead_code)]
    fn num_win_scenarios_brute(&self) -> usize {
        let mut count = 0;

        for time_wait in self.time/4..self.time {
            if (self.time * time_wait) - time_wait.pow(2) > self.distance {
                count += 1
            }
        }

        count
    }

    fn num_win_scenarios(&self) -> usize {
        let time = self.time as f64;
        let distance = self.distance as f64;

        let lower = (-time + (time.powf(2.0) - (4.0 * distance)).sqrt()) / -2.0;
        let lower = lower.floor();
        let upper = (-time - (time.powf(2.0) - (4.0 * distance)).sqrt()) / -2.0;
        let upper = upper.ceil();
        let result = upper - lower - 1.0;

        (result as i64) as usize
    }
}

fn parse_racedata(stdin: &Stdin) -> RaceData {
    let mut lines = stdin.lock().lines();

    let time = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    RaceData { time, distance }
}

fn main() {
    let racedata = parse_racedata(&stdin());
    // let solution = racedata.num_win_scenarios_brute();
    let solution = racedata.num_win_scenarios();
    println!("{solution}");
}
