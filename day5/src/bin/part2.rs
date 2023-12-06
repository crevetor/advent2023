use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
struct MapTypeErr;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum MapType {
    Seed,
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
            Seed => SeedToSoil,
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

struct Range {
    start: usize,
    end: usize,
}

struct MyMap {
    dst: Range,
    src: Range,
    len: usize,
}

impl MyMap {
    fn new(dst_start: usize, src_start: usize, len: usize) -> Self {
        let dst_end = dst_start + len - 1;
        let src_end = src_start + len - 1;
        MyMap {
            dst: Range {
                start: dst_start,
                end: dst_end,
            },
            src: Range {
                start: src_start,
                end: src_end,
            },
            len,
        }
    }

    fn to_src(self: &Self, v: usize) -> usize {
        self.src.start + v - self.dst.start
    }

    fn to_dst(self: &Self, v: usize) -> usize {
        self.dst.start + v - self.src.start
    }
}

impl From<(usize, usize, usize)> for MyMap {
    fn from(t: (usize, usize, usize)) -> Self {
        MyMap::new(t.0, t.1, t.2)
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

fn len(start: usize, end: usize) -> usize {
    end - start + 1
}

fn merge_ranges(
    range1: &(usize, usize, usize),
    range2: &(usize, usize, usize),
) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    let r1 = MyMap::from(*range1);
    let r2 = MyMap::from(*range2);

    // Ranges don't overlap
    if (r1.dst.end < r2.src.start) || (r1.dst.start > r2.src.end) {
        return ret;
    }

    if r1.dst.start < r2.src.start {
        ret.push((
            r2.dst.start,
            r1.to_src(r2.src.start),
            min(r2.len, len(r2.src.start, r1.dst.end)),
        ));
    } else if r1.dst.start == r2.src.start {
        ret.push((r2.dst.start, r1.dst.start, min(r1.len, r2.len)));
    } else {
        ret.push((
            r2.to_dst(r1.dst.start),
            r1.src.start,
            min(r1.len, len(r1.dst.start, r2.src.end)),
        ));
    }

    ret
}

fn reduce_map(
    curmap: &Vec<(usize, usize, usize)>,
    nextmap: &Vec<(usize, usize, usize)>,
) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();

    for range1 in curmap {
        let mut inter = Vec::new();
        for range2 in nextmap {
            inter.append(&mut merge_ranges(range1, range2));
        }
        // TODO: re-add the 1->1 ranges
        if inter.is_empty() {
            ret.push(*range1);
        } else {
            ret.append(&mut inter);
        }
    }

    ret
}

fn reduce_maps(maps: &HashMap<MapType, Vec<(usize, usize, usize)>>) -> Vec<(usize, usize, usize)> {
    let mut maptype = MapType::Seed;
    let mut curmap = maps.get(&maptype).unwrap().clone();
    loop {
        maptype = maptype.next_map();
        if maptype == MapType::Unknown {
            break;
        }
        curmap = reduce_map(&curmap, &maps.get(&maptype).unwrap());
    }
    curmap
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

    maps.insert(MapType::Seed, Vec::new());
    let mut seeds_iter = seeds.iter();
    loop {
        if let Some(start) = seeds_iter.next() {
            let len = seeds_iter.next().unwrap();
            if let Some(seed_ranges) = maps.get_mut(&MapType::Seed) {
                seed_ranges.push((*start, *start, *len));
            }
        } else {
            break;
        }
    }
    println!("{:?}", maps);

    let mut ranges = reduce_maps(&maps);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", ranges);
    println!("{}", ranges[0].0);
}
