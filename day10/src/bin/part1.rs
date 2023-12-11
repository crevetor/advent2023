use ndarray::Array2;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pipe {
    endpoints: [Direction; 4],
    position: [usize; 2],
    start: bool,
    ground: bool,
}

impl Pipe {
    fn new(c: char, x: usize, y: usize) -> Self {
        let endpoints = match c {
            '|' => [
                Direction::North,
                Direction::South,
                Direction::Unknown,
                Direction::Unknown,
            ],
            '-' => [
                Direction::East,
                Direction::West,
                Direction::Unknown,
                Direction::Unknown,
            ],
            'L' => [
                Direction::North,
                Direction::East,
                Direction::Unknown,
                Direction::Unknown,
            ],
            'J' => [
                Direction::North,
                Direction::West,
                Direction::Unknown,
                Direction::Unknown,
            ],
            '7' => [
                Direction::South,
                Direction::West,
                Direction::Unknown,
                Direction::Unknown,
            ],
            'F' => [
                Direction::South,
                Direction::East,
                Direction::Unknown,
                Direction::Unknown,
            ],
            'S' => [
                Direction::South,
                Direction::East,
                Direction::North,
                Direction::West,
            ],
            _ => [
                Direction::Unknown,
                Direction::Unknown,
                Direction::Unknown,
                Direction::Unknown,
            ],
        };
        Pipe {
            endpoints,
            position: [x, y],
            start: c == 'S',
            ground: c == '.',
        }
    }

    fn is_compatible(&self, other: &Self) -> bool {
        for endpoint in self.endpoints {
            match endpoint {
                Direction::North => {
                    if other.position[1] + 1 == self.position[1]
                        && other.endpoints.contains(&Direction::South)
                    {
                        return true;
                    }
                }
                Direction::South => {
                    if other.position[1] == self.position[1] + 1
                        && other.endpoints.contains(&Direction::North)
                    {
                        return true;
                    }
                }
                Direction::East => {
                    if other.position[0] == self.position[0] + 1
                        && other.endpoints.contains(&Direction::West)
                    {
                        return true;
                    }
                }
                Direction::West => {
                    if other.position[0] + 1 == self.position[0]
                        && other.endpoints.contains(&Direction::East)
                    {
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
    }
}

#[derive(Debug)]
struct Maze {
    contents: Array2<Pipe>,
    start: [usize; 2],
}

impl Maze {
    fn new(lines: &Vec<String>) -> Self {
        let mut pipes: Vec<Pipe> = Vec::new();
        let mut start = [0, 0];
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                pipes.push(Pipe::new(char, x, y));
                if char == 'S' {
                    start = [x, y];
                }
            }
        }

        Maze {
            contents: Array2::from_shape_vec((lines.len(), lines[0].chars().count()), pipes)
                .unwrap(),
            start,
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<[usize; 2]> {
        let mut ret = Vec::new();

        if x > 0 {
            ret.push([x - 1, y]);
        }

        if y > 0 {
            ret.push([x, y - 1]);
        }

        if x + 1 < self.contents.shape()[0] {
            ret.push([x + 1, y]);
        }

        if y + 1 < self.contents.shape()[1] {
            ret.push([x, y + 1]);
        }

        ret
    }

    fn path_len(&self) -> usize {
        let mut len = 1;

        let neighbors = self.get_neighbors(self.start[0], self.start[1]);
        let start: Pipe = *self.contents.get((self.start[1], self.start[0])).unwrap();
        let nexts: Vec<[usize; 2]> = neighbors
            .iter()
            .filter(|x| start.is_compatible(self.contents.get((x[1], x[0])).unwrap()))
            .cloned()
            .collect();

        let mut cur = start;
        let mut next: Pipe = *self.contents.get((nexts[0][1], nexts[0][0])).unwrap();

        while next != start {
            let compatibles: Vec<Pipe> = self
                .get_neighbors(next.position[0], next.position[1])
                .iter()
                .filter(|x| !(x[0] == cur.position[0] && x[1] == cur.position[1]))
                .map(|x| self.contents.get((x[1], x[0])).unwrap())
                .filter(|x| next.is_compatible(x))
                .cloned()
                .collect();
            if compatibles.len() > 1 {
                panic!("More than one possible path");
            }
            cur = next;
            next = compatibles[0];
            len += 1;
        }

        len
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
    let maze = Maze::new(&contents);
    println!("{maze:?}");

    let pathlen = maze.path_len();
    println!("{}", pathlen / 2);
}
