use std::cmp::min;
use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};
use std::process;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum AlmanacMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

use AlmanacMapType::*;

impl From<&str> for AlmanacMapType {
    fn from(map_name: &str) -> Self {
        match map_name {
            "seed-to-soil" => SeedToSoil,
            "soil-to-fertilizer" => SoilToFertilizer,
            "fertilizer-to-water" => FertilizerToWater,
            "water-to-light" => WaterToLight,
            "light-to-temperature" => LightToTemperature,
            "temperature-to-humidity" => TemperatureToHumidity,
            "humidity-to-location" => HumidityToLocation,
            _ => panic!("unknown map name '{}'", map_name),
        }
    }
}

const ALMANAC_MAP_TYPES: [AlmanacMapType; 7] = [
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
];

#[derive(Clone, Copy)]
struct ExclusiveRange {
    begin: usize,
    end: usize,
    len: usize,
}

impl ExclusiveRange {
    fn new(begin: usize, end: usize) -> Self {
        Self {
            begin,
            end,
            len: end - begin,
        }
    }

    fn new_by_len(begin: usize, len: usize) -> Self {
        Self {
            begin,
            end: begin + len,
            len,
        }
    }

    fn contains(&self, value: usize) -> bool {
        value >= self.begin && value < self.end
    }

    #[allow(dead_code)]
    fn iter(&self) -> impl Iterator<Item = usize> {
        self.begin..self.end
    }
}

struct RangeConversionResult {
    unconverted: Option<ExclusiveRange>,
    converted: Option<ExclusiveRange>,
}

#[derive(Clone)]
struct AlmanacMap {
    dst_start: usize,
    src_start: usize,
    src_range: ExclusiveRange,
}

impl AlmanacMap {
    fn new(dst_start: usize, src_start: usize, range_length: usize) -> Self {
        AlmanacMap {
            dst_start,
            src_start,
            src_range: ExclusiveRange::new_by_len(src_start, range_length),
        }
    }

    fn convert(&self, input: usize) -> Option<usize> {
        if !self.src_range.contains(input) {
            return None;
        }

        let offset = input.checked_sub(self.src_start).unwrap();
        let conversion = self.dst_start.checked_add(offset).unwrap();

        Some(conversion)
    }

    fn convert_range(&self, input: ExclusiveRange) -> RangeConversionResult {
        let input_first = input.begin;
        let input_last = input.end.checked_sub(1).unwrap();
        let src_first = self.src_range.begin;
        let src_last = self.src_range.end.checked_sub(1).unwrap();

        // -- No intersection.
        if !self.src_range.contains(input_first) && !self.src_range.contains(input_last) {
            return RangeConversionResult {
                unconverted: Some(input),
                converted: None,
            };
        }

        // -- Same or subset.
        if self.src_range.contains(input_first) && self.src_range.contains(input_last) {
            let start = self.convert(input_first).unwrap();
            let end = self.convert(input_last).unwrap().checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: None,
                converted: Some(ExclusiveRange::new(start, end)),
            };
        }

        // -- Lesser offset.
        if !self.src_range.contains(input_first) && self.src_range.contains(input_last) {
            let start_unchanged = input_first;
            let end_unchanged = src_first;
            let start_converted = self.convert(src_first).unwrap();
            let end_converted = self.convert(input_last).unwrap().checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: Some(ExclusiveRange::new(start_unchanged, end_unchanged)),
                converted: Some(ExclusiveRange::new(start_converted, end_converted)),
            };
        }

        // -- Greater offset.
        if self.src_range.contains(input_first) && !self.src_range.contains(input_last) {
            let start_converted = self.convert(input_first).unwrap();
            let end_converted = self.convert(src_last).unwrap().checked_add(1).unwrap();
            let start_unchanged = src_last.checked_add(1).unwrap();
            let end_unchanged = input_last.checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: Some(ExclusiveRange::new(start_unchanged, end_unchanged)),
                converted: Some(ExclusiveRange::new(start_converted, end_converted)),
            };
        }

        panic!("unhandled range scenario");
    }
}

fn nproc() -> usize {
    let output = process::Command::new("nproc").output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    output.trim_end().parse().unwrap()
}

#[derive(Clone)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<AlmanacMapType, Vec<AlmanacMap>>,
}

impl From<&Stdin> for Almanac {
    fn from(stdin: &Stdin) -> Self {
        let mut lines = stdin.lock().lines();

        let seeds: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect();

        let mut maps: HashMap<_, _> = ALMANAC_MAP_TYPES
            .iter()
            .map(|map_type| (*map_type, Vec::new()))
            .collect();

        let mut curr_type = None;

        while let Some(Ok(line)) = lines.next() {
            // -- Skip blank lines.
            if line.is_empty() {
                continue;
            }

            // -- Switch building map.
            if line.ends_with(" map:") {
                let map_name = line.split_whitespace().next().unwrap();
                curr_type = Some(AlmanacMapType::from(map_name));
                continue;
            }

            // -- Create new almanac map for input.
            let mut nums = line.split_whitespace().map(|w| w.parse().unwrap());
            let dst_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let range_length = nums.next().unwrap();
            let almanac_map = AlmanacMap::new(dst_start, src_start, range_length);

            match curr_type {
                Some(curr_type) => maps.get_mut(&curr_type).unwrap().push(almanac_map),
                None => panic!("attempt to push to curr_type when None"),
            }
        }

        Self { seeds, maps }
    }
}

impl Almanac {
    fn transform_with_map(&self, map_type: &AlmanacMapType, value: usize) -> usize {
        let maps = self.maps.get(map_type).unwrap();

        for map in maps {
            if let Some(converted) = map.convert(value) {
                return converted;
            }
        }

        value
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let mut location = seed;

        for map_type in ALMANAC_MAP_TYPES.iter() {
            location = self.transform_with_map(map_type, location);
        }

        location
    }

    fn range_to_location(&self, range: ExclusiveRange) -> usize {
        let mut lowest = usize::MAX;

        for seed in range.iter() {
            let location = self.seed_to_location(seed);
            lowest = min(lowest, location);
        }

        lowest
    }

    fn get_seed_ranges(&self) -> Vec<ExclusiveRange> {
        self.seeds
            .windows(2)
            .step_by(2)
            .map(|w| ExclusiveRange::new_by_len(w[0], w[1]))
            .collect()
    }

    #[allow(dead_code)]
    fn lowest_location_brute_single(&self) -> usize {
        let ranges = self.get_seed_ranges();
        let nranges = ranges.len() - 1;
        let mut lowest = usize::MAX;

        for (index, range) in ranges.iter().enumerate() {
            println!(
                "range #{:2} / {:2}, {:10} -> {:10}, size: {:10}",
                index, nranges, range.begin, range.end, range.len
            );

            lowest = min(lowest, self.range_to_location(*range));
        }

        lowest
    }

    #[allow(dead_code)]
    fn lowest_location_brute_threaded(&self) -> usize {
        let nproc = nproc();
        let mut ranges = self.get_seed_ranges();
        let mut handles = Vec::new();
        let mut lowest = usize::MAX;

        while !ranges.is_empty() || !handles.is_empty() {
            while handles.len() < nproc {
                if ranges.is_empty() {
                    break;
                }

                let clone = (*self).clone();
                let range = ranges.pop().unwrap();

                println!(
                    "starting thread for range {:10} -> {:10}, size {:10}",
                    range.begin, range.end, range.len,
                );

                handles.push(thread::spawn(move || clone.range_to_location(range)));
            }

            thread::sleep(Duration::from_secs(3));

            let mut todo = Vec::new();
            let mut done = Vec::new();

            while let Some(handle) = handles.pop() {
                if handle.is_finished() {
                    done.push(handle);
                } else {
                    todo.push(handle);
                }
            }

            for t in done {
                lowest = min(lowest, t.join().unwrap());
            }

            handles = todo;
        }

        lowest
    }

    fn lowest_location(&self) -> usize {
        let mut ranges = self.get_seed_ranges();

        for map_type in ALMANAC_MAP_TYPES.iter() {
            let mut new_ranges = Vec::new();

            while let Some(range) = ranges.pop() {
                let mut was_converted = false;

                for map in self.maps.get(map_type).unwrap() {
                    let RangeConversionResult {
                        unconverted,
                        converted,
                    } = map.convert_range(range);

                    match (unconverted, converted) {
                        // -- No conversion.
                        (Some(_), None) => {}

                        // -- Total conversion.
                        (None, Some(converted)) => {
                            was_converted = true;
                            new_ranges.push(converted);
                            break;
                        }
                        // -- Partial.
                        (Some(unconverted), Some(converted)) => {
                            was_converted = true;
                            ranges.push(unconverted);
                            new_ranges.push(converted);
                            break;
                        }
                        // -- Error.
                        (None, None) => panic!("bad conversion state!"),
                    }
                }

                // -- Keep range if no conversion occured.
                if !was_converted {
                    new_ranges.push(range);
                }
            }

            ranges = new_ranges;
        }

        ranges.iter().map(|r| r.begin).min().unwrap()
    }
}

fn main() {
    let almanac = Almanac::from(&stdin());
    // let lowest = almanac.lowest_location_brute_single();
    // let lowest = almanac.lowest_location_brute_threaded();
    let lowest = almanac.lowest_location();
    println!("{lowest}");
}
