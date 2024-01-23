use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

struct Hailstone {
    pos: [f32; 3],
    vel: [f32; 3],
}

#[derive(Debug)]
struct HailstoneErr;
impl FromStr for Hailstone {
    type Err = HailstoneErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" @ ").unwrap();
        Ok(Hailstone {
            pos: left
                .split(", ")
                .map(|x| f32::from_str(x.trim_start()).unwrap())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
            vel: right
                .split(", ")
                .map(|x| f32::from_str(x.trim_start()).unwrap())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl Hailstone {
    fn intersection(&self, other: &Self) -> Option<[f32; 2]> {
        if (self.vel[1] == 0.0 && other.vel[1] == 0.0)
            || (self.vel[0] / self.vel[1] == other.vel[0] / other.vel[1])
        {
            None
        } else {
            let r = (self.pos[0] * other.vel[1] - self.pos[1] * other.vel[0]
                + other.pos[1] * other.vel[0]
                - other.pos[0] * other.vel[1])
                / (self.vel[1] * other.vel[0] - self.vel[0] * other.vel[1]);
            let s = (self.pos[0] - other.pos[0] + r * self.vel[0]) / other.vel[0];

            if r > 0.0 && s > 0.0 {
                Some([self.pos[0] + r * self.vel[0], self.pos[1] + r * self.vel[1]])
            } else {
                None
            }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let hailstones: Vec<Hailstone> = contents
        .iter()
        .map(|x| Hailstone::from_str(x).unwrap())
        .collect();

    let mut intersections = Vec::new();
    let mut intersects = 0;
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Some(intersection) = hailstones[i].intersection(&hailstones[j]) {
                if intersection[0] >= min
                    && intersection[0] <= max
                    && intersection[1] >= min
                    && intersection[1] <= max
                {
                    intersects += 1;
                }
            }
            intersections.push((i, j, hailstones[i].intersection(&hailstones[j])));
        }
    }
    println!("{intersections:?}");
    println!("{intersects}");
}
