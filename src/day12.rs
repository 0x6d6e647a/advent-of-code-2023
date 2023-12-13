use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::{cmp::min, collections::HashMap};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

use SpringState::*;

impl From<char> for SpringState {
    fn from(c: char) -> Self {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("invalid character for SpringState: '{}'", c),
        }
    }
}

struct SpringRecord {
    springs: Vec<SpringState>,
    checksum: Vec<u8>,
}

impl SpringRecord {
    fn new(line: &str) -> SpringRecord {
        let mut components = line.split_whitespace();
        let springs = components
            .next()
            .unwrap()
            .chars()
            .map(|c| c.into())
            .collect();
        let checksum = components
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        SpringRecord { springs, checksum }
    }

    fn unfold(&self) -> SpringRecord {
        let mut springs = self.springs.clone();
        let mut checksum = self.checksum.clone();

        (0..4).for_each(|_| {
            springs.push(Unknown);
            springs.extend(self.springs.iter());
            checksum.extend(self.checksum.iter());
        });

        SpringRecord { springs, checksum }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<SpringRecord> {
    input.lines().map(SpringRecord::new).collect()
}

type ArrangementCache<'a> = HashMap<u64, usize>;

fn get_hash(springs: &[SpringState], checksum: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();

    hasher.write_usize(springs.len());
    springs.iter().for_each(|s| hasher.write_u8(*s as u8));
    hasher.write_usize(checksum.len());
    checksum.iter().for_each(|b| hasher.write_u8(*b));

    hasher.finish()
}

fn count_arrangments(
    cache: &mut ArrangementCache,
    springs: &[SpringState],
    checksum: &[u8],
) -> usize {
    // -- No more springs.
    if springs.is_empty() {
        // -- Checksum exhausted, valid configuration.
        if checksum.is_empty() {
            return 1;
        }
        // -- Invalid configuration.
        return 0;
    }

    // -- Checksum exhausted.
    if checksum.is_empty() {
        // -- No damaged springs, valid configuration.
        if !springs.contains(&Damaged) {
            return 1;
        }
        // -- Invalid configuration.
        return 0;
    }

    // -- Use cached result if possible.
    let key = get_hash(springs, checksum);

    if let Some(arrangements) = cache.get(&key) {
        return *arrangements;
    }

    // -- Try potential arrangments.
    let mut arrangments = 0;

    if matches!(springs[0], Operational | Unknown) {
        arrangments += count_arrangments(cache, &springs[1..], checksum);
    }

    if matches!(springs[0], Damaged | Unknown) {
        // -- Check if enough springs to satisfy next checksum.
        if checksum[0] as usize <= springs.len() {
            // -- Check that no operational springs to satisfy next checksum.
            if !springs[..checksum[0] as usize].contains(&Operational) {
                // -- Check that no remaining springs left after satifying checksum.
                // -- OR
                // -- Check the that next spring is not damanged.
                if (checksum[0] as usize == springs.len())
                    || (springs[checksum[0] as usize] != Damaged)
                {
                    let crop = min(checksum[0] as usize + 1, springs.len());
                    arrangments += count_arrangments(cache, &springs[crop..], &checksum[1..])
                }
            }
        }
    }

    cache.insert(key, arrangments);
    arrangments
}

#[aoc(day12, part1)]
fn part1(spring_records: &[SpringRecord]) -> usize {
    let mut cache = HashMap::new();
    spring_records
        .iter()
        .map(|sr| count_arrangments(&mut cache, &sr.springs, &sr.checksum))
        .sum()
}

#[aoc(day12, part2)]
fn part2(spring_records: &[SpringRecord]) -> usize {
    let mut cache = HashMap::new();
    spring_records
        .iter()
        .map(|sr| sr.unfold())
        .map(|sr| count_arrangments(&mut cache, &sr.springs, &sr.checksum))
        .sum()
}
