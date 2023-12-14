use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

#[derive(Clone, Copy)]
enum TiltDirection {
    North,
}

trait Platform {
    fn tilt(&mut self, direction: TiltDirection) -> usize;
    fn move_rock(&mut self, direction: TiltDirection, x: usize, y: usize) -> (usize, usize);
}

impl Platform for Matrix<char> {
    fn tilt(&mut self, direction: TiltDirection) -> usize {
        for y in 1..self.num_rows() {
            for x in 0..self.num_cols() {
                let (new_x, new_y) = self.move_rock(direction, x, y);
                if new_y != y {
                    self.set(x, y, '.');
                    self.set(new_x, new_y, 'O');
                }
            }
        }

        self.rows().rev().enumerate().fold(0, |acc, e| {
            acc + ((e.0 + 1) * e.1.iter().filter(|c| c == &&'O').count())
        })
    }

    fn move_rock(&mut self, direction: TiltDirection, x: usize, y: usize) -> (usize, usize) {
        if self.get(x, y) == Some('.') || self.get(x, y) == Some('#') {
            return (x, y);
        }

        let mut new_y = y;

        loop {
            new_y = new_y - 1;

            if self.get(x, new_y) == Some('#') || self.get(x, new_y) == Some('O') {
                return (x, new_y + 1);
            }
            if new_y == 0 {
                return (x, 0);
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

fn parse_input(contents: &Vec<String>) -> Matrix<char> {
    contents.iter().map(|x| x.chars().collect()).collect()
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
    println!("{}", platform.tilt(TiltDirection::North));
    println!("{platform}");
}
