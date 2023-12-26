use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};

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

struct ExclusiveRange {
    begin: usize,
    end: usize,
}

impl ExclusiveRange {
    fn new(begin: usize, len: usize) -> Self {
        Self {
            begin,
            end: begin + len,
        }
    }

    fn contains(&self, value: usize) -> bool {
        value >= self.begin && value < self.end
    }
}

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
            src_range: ExclusiveRange::new(src_start, range_length),
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
}

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

    fn lowest_location(&self) -> usize {
        let mut values = self.seeds.clone();

        for map_type in ALMANAC_MAP_TYPES.iter() {
            let mut new_values = Vec::new();

            for value in values {
                new_values.push(self.transform_with_map(map_type, value));
            }

            values = new_values;
        }

        values.into_iter().min().unwrap()
    }
}

fn main() {
    let almanac = Almanac::from(&stdin());
    let lowest = almanac.lowest_location();
    println!("{lowest}");
}
