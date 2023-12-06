use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
struct MapTypeErr;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Unknown,
}

impl FromStr for MapType {
    type Err = MapTypeErr;

    fn from_str(s: &str) -> Result<MapType, Self::Err> {
        match s {
            "seed-to-soil" => Ok(MapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapType::SoilToFertilizer),
            "fertilizer-to-water" => Ok(MapType::FertilizerToWater),
            "water-to-light" => Ok(MapType::WaterToLight),
            "light-to-temperature" => Ok(MapType::LightToTemperature),
            "temperature-to-humidity" => Ok(MapType::TemperatureToHumidity),
            "humidity-to-location" => Ok(MapType::HumidityToLocation),
            _ => Err(MapTypeErr),
        }
    }
}

impl MapType {
    fn next_map(self: &Self) -> MapType {
        use MapType::*;
        match *self {
            SeedToSoil => SoilToFertilizer,
            SoilToFertilizer => FertilizerToWater,
            FertilizerToWater => WaterToLight,
            WaterToLight => LightToTemperature,
            LightToTemperature => TemperatureToHumidity,
            TemperatureToHumidity => HumidityToLocation,
            HumidityToLocation => Unknown,
            Unknown => Unknown,
        }
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn map_value(v: usize, ranges: &Vec<(usize, usize, usize)>) -> usize {
    for range in ranges {
        if v < range.1 {
            continue;
        } else {
            if v <= range.1 + range.2 - 1 {
                return range.0 + (v - range.1);
            }
        }
    }
    v
}

fn process_seed(seed: usize, maps: &HashMap<MapType, Vec<(usize, usize, usize)>>) -> usize {
    let mut maptype = MapType::SeedToSoil;
    let mut newseed = seed;
    while maptype != MapType::Unknown {
        newseed = map_value(newseed, maps.get(&maptype).unwrap());
        maptype = maptype.next_map();
    }
    newseed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let seeds: Vec<usize> = contents
        .iter()
        .next()
        .expect("No lines in contents")
        .split_once(": ")
        .expect("Couldn't split on ': '")
        .1
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps: HashMap<MapType, Vec<(usize, usize, usize)>> = HashMap::new();

    let mut curmap = MapType::Unknown;
    for line in contents {
        if line.is_empty() {
            continue;
        }
        if line.contains("map:") {
            curmap = line
                .split_once(" ")
                .expect("Couldn't split map name")
                .0
                .parse()
                .unwrap();
            maps.insert(curmap, Vec::new());
        } else {
            let mut range = line.splitn(3, ' ').map(|x| x.parse().unwrap());
            if let Some(v) = maps.get_mut(&curmap) {
                v.push((
                    range.next().unwrap(),
                    range.next().unwrap(),
                    range.next().unwrap(),
                ));
            }
        }
    }
    println!("{:?}", maps);

    let mut soils: Vec<usize> = Vec::new();
    for seed in seeds {
        soils.push(process_seed(seed, &maps));
    }
    println!("{:?}", soils);
    println!("{}", soils.iter().min().unwrap());
}
