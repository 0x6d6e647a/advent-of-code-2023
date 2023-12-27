use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{stdin, BufRead};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

use SpringState::*;

impl From<char> for SpringState {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("invalid character for SpringState: '{}'", ch),
        }
    }
}

struct SpringRecord {
    springs: Vec<SpringState>,
    checksum: Vec<u8>,
}

impl From<String> for SpringRecord {
    fn from(line: String) -> Self {
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
        Self { springs, checksum }
    }
}

type ArrangementCache<'a> = HashMap<u64, usize>;

fn get_hash(springs: &[SpringState], checksum: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    springs.hash(&mut hasher);
    checksum.hash(&mut hasher);
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

fn main() {
    let spring_records: Vec<SpringRecord> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().into())
        .collect();
    let mut cache = HashMap::new();
    let sum: usize = spring_records
        .iter()
        .map(|sr| count_arrangments(&mut cache, &sr.springs, &sr.checksum))
        .sum();
    println!("{sum}");
}
