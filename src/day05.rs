use std::cmp::min;
use std::process;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AlmanacMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl From<&str> for AlmanacMapType {
    fn from(map_name: &str) -> Self {
        use AlmanacMapType::*;
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
    AlmanacMapType::SeedToSoil,
    AlmanacMapType::SoilToFertilizer,
    AlmanacMapType::FertilizerToWater,
    AlmanacMapType::WaterToLight,
    AlmanacMapType::LightToTemperature,
    AlmanacMapType::TemperatureToHumidity,
    AlmanacMapType::HumidityToLocation,
];

struct RangeConversionResult {
    unconverted: Option<Range<usize>>,
    converted: Option<Range<usize>>,
}

#[derive(Debug, Clone)]
struct AlmanacMap {
    dst_start: usize,
    src_start: usize,
    src_range: Range<usize>,
}

impl AlmanacMap {
    fn new(dst_start: usize, src_start: usize, range_length: usize) -> Self {
        let src_range = src_start..src_start + range_length;
        AlmanacMap {
            dst_start,
            src_start,
            src_range,
        }
    }

    fn convert(&self, input: usize) -> Option<usize> {
        if !self.src_range.contains(&input) {
            return None;
        }

        let offset = input.checked_sub(self.src_start).unwrap();
        let conversion = self.dst_start.checked_add(offset).unwrap();

        Some(conversion)
    }

    fn convert_range(&self, input: Range<usize>) -> RangeConversionResult {
        let input_first = input.start;
        let input_last = input.end.checked_sub(1).unwrap();
        let src_first = self.src_range.start;
        let src_last = self.src_range.end.checked_sub(1).unwrap();

        // -- No intersection.
        if !self.src_range.contains(&input_first) && !self.src_range.contains(&input_last) {
            return RangeConversionResult {
                unconverted: Some(input),
                converted: None,
            };
        }

        // -- Same or subset.
        if self.src_range.contains(&input_first) && self.src_range.contains(&input_last) {
            let start = self.convert(input_first).unwrap();
            let end = self.convert(input_last).unwrap().checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: None,
                converted: Some(start..end),
            };
        }

        // -- Lesser offset.
        if !self.src_range.contains(&input_first) && self.src_range.contains(&input_last) {
            let start_unchanged = input_first;
            let end_unchanged = src_first;
            let start_converted = self.convert(src_first).unwrap();
            let end_converted = self.convert(input_last).unwrap().checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: Some(start_unchanged..end_unchanged),
                converted: Some(start_converted..end_converted),
            };
        }

        // -- Greater offset.
        if self.src_range.contains(&input_first) && !self.src_range.contains(&input_last) {
            let start_converted = self.convert(input_first).unwrap();
            let end_converted = self.convert(src_last).unwrap().checked_add(1).unwrap();
            let start_unchanged = src_last.checked_add(1).unwrap();
            let end_unchanged = input_last.checked_add(1).unwrap();
            return RangeConversionResult {
                unconverted: Some(start_unchanged..end_unchanged),
                converted: Some(start_converted..end_converted),
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

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<AlmanacMapType, Vec<AlmanacMap>>,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();

        let seeds: Vec<_> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect();

        use AlmanacMapType::*;
        let mut maps = HashMap::from([
            (SeedToSoil, Vec::new()),
            (SoilToFertilizer, Vec::new()),
            (FertilizerToWater, Vec::new()),
            (WaterToLight, Vec::new()),
            (LightToTemperature, Vec::new()),
            (TemperatureToHumidity, Vec::new()),
            (HumidityToLocation, Vec::new()),
        ]);

        let mut curr_map = None;

        for line in lines {
            // -- Skip blank lines.
            if line.is_empty() {
                continue;
            }

            // -- Switch building map.
            if line.ends_with(" map:") {
                let map_name = line.split_whitespace().next().unwrap();
                let curr_type = Some(AlmanacMapType::from(map_name));
                curr_map = Some(maps.get_mut(curr_type.as_ref().unwrap()).unwrap());
                continue;
            }

            // -- Create new almanac map for input.
            let mut nums = line.split_whitespace().map(|w| w.parse().unwrap());
            let dst_start = nums.next().unwrap();
            let src_start = nums.next().unwrap();
            let range_length = nums.next().unwrap();
            let almanac_map = AlmanacMap::new(dst_start, src_start, range_length);

            match curr_map {
                Some(ref mut curr_map) => curr_map.push(almanac_map),
                None => panic!("attempt to push to curr_map when None"),
            };
        }

        Almanac { seeds, maps }
    }

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

    fn range_to_location(&self, range: Range<usize>) -> usize {
        let mut lowest = usize::MAX;

        for seed in range {
            let location = self.seed_to_location(seed);
            lowest = min(lowest, location);
        }

        lowest
    }

    fn get_seed_ranges(&self) -> Vec<Range<usize>> {
        self.seeds
            .windows(2)
            .step_by(2)
            .map(|w| w[0]..w[0] + w[1])
            .collect()
    }

    #[allow(dead_code)]
    fn part2_brute_single(&self) -> usize {
        let ranges = self.get_seed_ranges();
        let nranges = ranges.len() - 1;
        let mut lowest = usize::MAX;

        for (index, range) in ranges.iter().enumerate() {
            println!(
                "range #{:2} / {:2}, {:10} -> {:10}, size: {:10}",
                index,
                nranges,
                range.start,
                range.end,
                range.len()
            );

            lowest = min(lowest, self.range_to_location(range.clone()));
        }

        lowest
    }

    #[allow(dead_code)]
    fn part2_brute_threaded(&self) -> usize {
        let nproc = nproc();
        let mut ranges = self.get_seed_ranges();
        let mut handles = Vec::new();
        let mut lowest = usize::MAX;

        while !ranges.is_empty() || !handles.is_empty() {
            while handles.len() < nproc {
                if ranges.is_empty() {
                    break;
                }

                let clone = self.clone();
                let range = ranges.pop().unwrap();

                println!(
                    "starting thread for range {:10} -> {:10}, size {:10}",
                    range.start,
                    range.end,
                    range.len()
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

    fn part2(&self) -> usize {
        let mut ranges = self.get_seed_ranges();

        for map_type in ALMANAC_MAP_TYPES.iter() {
            let mut new_ranges = Vec::new();

            while let Some(range) = ranges.pop() {
                let mut was_converted = false;

                for map in self.maps.get(map_type).unwrap() {
                    let RangeConversionResult {
                        unconverted,
                        converted,
                    } = map.convert_range(range.clone());

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

        ranges.iter().map(|r| r.start).min().unwrap()
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    Almanac::new(input)
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> usize {
    let mut values = almanac.seeds.clone();

    for map_type in ALMANAC_MAP_TYPES.iter() {
        let mut new_values = Vec::new();

        for value in values {
            new_values.push(almanac.transform_with_map(map_type, value));
        }

        values = new_values;
    }

    *values.iter().min().unwrap()
}

#[aoc(day5, part2)]
fn part2(almanac: &Almanac) -> usize {
    // almanac.part2_brute_single()
    // almanac.part2_brute_threaded()
    almanac.part2()
}
