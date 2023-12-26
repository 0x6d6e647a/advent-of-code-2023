use std::io::{stdin, BufRead, Stdin};
use std::iter::zip;

struct RaceData {
    time: usize,
    distance: usize,
}

impl RaceData {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn num_win_scenarios(&self) -> usize {
        let mut total = 0;

        for time_wait in 1..self.time {
            let time_go = self.time - time_wait;

            if time_wait * time_go > self.distance {
                total += 1;
            }
        }

        total
    }
}

fn parse_racedata(stdin: &Stdin) -> Vec<RaceData> {
    let mut lines = stdin.lock().lines();

    let times: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(times.len(), distances.len());

    zip(times, distances)
        .map(|(t, d)| RaceData::new(t, d))
        .collect()
}

fn main() {
    let racedata = parse_racedata(&stdin());
    let solution: usize = racedata
        .into_iter()
        .map(|rd| rd.num_win_scenarios())
        .product();
    println!("{solution}");
}
