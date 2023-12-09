use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let first = solve_first(&input);
    let second = solve_second(&input);

    (first.to_string(), second.to_string())
}

fn solve_first(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let seeds: Vec<u64> = lines[0][7..]
        .split(' ')
        .map(|s| {
            s.parse()
                .expect(format!("seed must be a number but is '{}'", s).as_str())
        })
        .collect();

    let almanac = parse_almanac(lines);

    let lowest_location = seeds
        .iter()
        .map(|s| lookup_location(*s, &almanac))
        .min()
        .unwrap();

    lowest_location
}

fn lookup_location(seed: u64, almanac: &Almanac) -> u64 {
    let soil = almanac[&MapType::SeedToSoil]
        .iter()
        .find_map(|map| map.try_map(seed))
        .unwrap_or(seed);
    let fertilizer = almanac[&MapType::SoilToFertilizer]
        .iter()
        .find_map(|map| map.try_map(soil))
        .unwrap_or(soil);
    let water = almanac[&MapType::FertilizerToWater]
        .iter()
        .find_map(|map| map.try_map(fertilizer))
        .unwrap_or(fertilizer);
    let light = almanac[&MapType::WaterToLight]
        .iter()
        .find_map(|map| map.try_map(water))
        .unwrap_or(water);
    let temperature = almanac[&MapType::LightToTemperature]
        .iter()
        .find_map(|map| map.try_map(light))
        .unwrap_or(light);
    let humidity = almanac[&MapType::TemperatureToHumidity]
        .iter()
        .find_map(|map| map.try_map(temperature))
        .unwrap_or(temperature);
    let location = almanac[&MapType::HumidityToLocation]
        .iter()
        .find_map(|map| map.try_map(humidity))
        .unwrap_or(humidity);

    location
}

fn parse_almanac(lines: Vec<&str>) -> Almanac {
    let mut parse_state = ParseState {
        current_map_type: None,
        almanac: HashMap::with_capacity(7),
    };

    for line in &lines[1..] {
        if line.trim().is_empty() {
            parse_state.current_map_type = None;
        } else if line.chars().next().unwrap().is_alphabetic() {
            let new_map_type = parse_map_type(line.trim_end_matches(" map:"));
            parse_state.current_map_type = new_map_type;
            parse_state
                .almanac
                .insert(parse_state.current_map_type.unwrap(), vec![]);
        } else if line.chars().next().unwrap().is_numeric() {
            if parse_state.current_map_type.is_none() {
                panic!("line starts with numeric but no map type is set");
            }
            let map = match line
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()[..]
            {
                [destination, source, range] => Map {
                    source,
                    destination,
                    range,
                },
                _ => panic!("there must be three numbers"),
            };

            if let Some(maps) = parse_state
                .almanac
                .get_mut(&parse_state.current_map_type.unwrap())
            {
                maps.push(map);
            }
        } else {
            panic!("line starts with unexpected character");
        }
    }

    parse_state.almanac
}

type Almanac = HashMap<MapType, Vec<Map>>;

#[derive(Debug)]
struct ParseState {
    current_map_type: Option<MapType>,
    almanac: HashMap<MapType, Vec<Map>>,
}

fn solve_second(_input: &str) -> i32 {
    0
}

fn parse_map_type(input: &str) -> Option<MapType> {
    match input {
        "seed-to-soil" => Some(MapType::SeedToSoil),
        "soil-to-fertilizer" => Some(MapType::SoilToFertilizer),
        "fertilizer-to-water" => Some(MapType::FertilizerToWater),
        "water-to-light" => Some(MapType::WaterToLight),
        "light-to-temperature" => Some(MapType::LightToTemperature),
        "temperature-to-humidity" => Some(MapType::TemperatureToHumidity),
        "humidity-to-location" => Some(MapType::HumidityToLocation),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Map {
    source: u64,
    destination: u64,
    range: u64,
}

impl Map {
    fn try_map(&self, value: u64) -> Option<u64> {
        if value >= self.source && value < self.source + self.range {
            let mapped = self.destination + 1 * (value - self.source);
            return Some(mapped);
        }
        None
    }
}
