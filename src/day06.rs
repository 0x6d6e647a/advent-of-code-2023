use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct RaceData {
    time: usize,
    distance: usize,
}

impl RaceData {
    fn num_win_scenarios_p1(&self) -> usize {
        let mut total = 0;

        for time_wait in 1..self.time {
            let time_go = self.time - time_wait;

            if time_wait * time_go > self.distance {
                total += 1;
            }
        }

        total
    }

    #[allow(dead_code)]
    fn num_win_scenarios_p2_brute(&self) -> usize {
        let mut count = 0;

        for time_wait in self.time/4..self.time {
            if (self.time * time_wait) - time_wait.pow(2) > self.distance {
                count += 1
            }
        }

        count
    }

    fn num_win_scenarios_p2(&self) -> usize {
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

#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<RaceData>, RaceData) {
    let mut lines = input.lines();
    let times: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(times.len(), distances.len());

    // -- Part 1
    let mut racedatas_p1 = Vec::new();

    let mut times_p1 = times.iter();
    let mut distances_p1 = distances.iter();

    for _ in 0..times_p1.len() {
        let time = *times_p1.next().unwrap();
        let distance = *distances_p1.next().unwrap();

        racedatas_p1.push(RaceData { time, distance })
    }

    // -- Part 2
    let mut lines = input.lines();
    let time = lines
        .next()
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
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let racedata_p2 = RaceData { time, distance };

    (racedatas_p1, racedata_p2)
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<RaceData>, RaceData)) -> usize {
    let (input, _) = input;
    input.iter().map(|rd| rd.num_win_scenarios_p1()).product()
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<RaceData>, RaceData)) -> usize {
    let (_, input) = input;
    input.num_win_scenarios_p2()
}
