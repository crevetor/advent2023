use matrix::Matrix;
use std::env;
use std::fs;
use std::ops::Index;
use std::process;

#[derive(Clone, Copy)]
enum TiltDirection {
    North = 1,
    West,
    South,
    East,
}

impl TiltDirection {
    fn next(&self) -> Option<Self> {
        match *self {
            TiltDirection::North => Some(TiltDirection::West),
            TiltDirection::West => Some(TiltDirection::South),
            TiltDirection::South => Some(TiltDirection::East),
            TiltDirection::East => None,
        }
    }
}

trait Platform {
    fn tilt(&mut self, direction: TiltDirection);
    fn move_rock(&mut self, direction: TiltDirection, x: usize, y: usize) -> (usize, usize);
    fn cycle(&mut self);
    fn load(&self) -> usize;
}

impl Platform for Matrix<char> {
    fn tilt(&mut self, direction: TiltDirection) {
        let y_range: Vec<_> = match direction {
            TiltDirection::North => (1..self.num_rows()).collect(),
            TiltDirection::South => (0..self.num_rows() - 1).rev().collect(),
            _ => (0..self.num_rows()).collect(),
        };

        let x_range: Vec<_> = match direction {
            TiltDirection::West => (1..self.num_cols()).collect(),
            TiltDirection::East => (0..self.num_cols() - 1).rev().collect(),
            _ => (0..self.num_cols()).collect(),
        };

        for y in y_range {
            for x in x_range.clone() {
                let (new_x, new_y) = self.move_rock(direction, x, y);
                if new_x != x || new_y != y {
                    self.set(x, y, '.');
                    self.set(new_x, new_y, 'O');
                }
            }
        }
    }

    fn move_rock(&mut self, direction: TiltDirection, x: usize, y: usize) -> (usize, usize) {
        if self.get(x, y) == Some('.') || self.get(x, y) == Some('#') {
            return (x, y);
        }

        let (x_transform, y_transform, x_stop, y_stop) = match direction {
            TiltDirection::North => (0, -1, 0, 0),
            TiltDirection::West => (-1, 0, 0, 0),
            TiltDirection::South => (0, 1, 0, self.num_rows() - 1),
            TiltDirection::East => (1, 0, self.num_cols() - 1, 0),
        };

        let mut new_y = i32::try_from(y).unwrap();
        let mut new_x = i32::try_from(x).unwrap();
        let mut old_x = new_x;
        let mut old_y = new_y;

        loop {
            old_x = new_x;
            old_y = new_y;
            new_x = new_x + x_transform;
            new_y = new_y + y_transform;

            if self.get(new_x.try_into().unwrap(), new_y.try_into().unwrap()) == Some('#')
                || self.get(new_x.try_into().unwrap(), new_y.try_into().unwrap()) == Some('O')
            {
                return (old_x.try_into().unwrap(), old_y.try_into().unwrap());
            }

            match direction {
                TiltDirection::North | TiltDirection::South => {
                    if new_y == y_stop.try_into().unwrap() {
                        return (x, y_stop);
                    }
                }
                TiltDirection::West | TiltDirection::East => {
                    if new_x == x_stop.try_into().unwrap() {
                        return (x_stop, y);
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        let mut direction = Some(TiltDirection::North);
        while direction.is_some() {
            self.tilt(direction.unwrap());
            direction = direction.unwrap().next();
        }
    }

    fn load(&self) -> usize {
        self.rows().rev().enumerate().fold(0, |acc, e| {
            acc + ((e.0 + 1) * e.1.iter().filter(|c| c == &&'O').count())
        })
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

fn parse_input(contents: &Vec<String>) -> Matrix<char> {
    contents.iter().map(|x| x.chars().collect()).collect()
}

fn find_pattern(idx: usize, loads: &Vec<usize>) -> Option<Vec<usize>> {
    let mut offset = 1;
    let pat_start = loads[idx];
    while loads[idx - offset] == loads[loads.len() - offset]
        && loads[loads.len() - offset] != pat_start
    {
        offset += 1;
    }
    if offset == 1 || loads[loads.len() - offset] != pat_start {
        return None;
    }

    Some(loads[loads.len() - offset..].iter().cloned().collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut platform = parse_input(&contents);
    println!("{platform}");

    let mut cycles = 0;
    let mut loads: Vec<usize> = Vec::new();

    loop {
        platform.cycle();
        cycles += 1;
        let load = platform.load();
        if loads.contains(&load) {
            if let Some(pattern) =
                find_pattern(loads.iter().position(|l| l == &load).unwrap(), &loads)
            {
                println!(
                    "Found pattern {pattern:?} of len {} at {}",
                    pattern.len(),
                    loads.iter().position(|l| l == &load).unwrap()
                );
                println!("{loads:?}");
                break;
            }
        }
        loads.push(load);
        if cycles == 1000000000 {
            break;
        }
    }
    println!("{}", platform.load());
}
