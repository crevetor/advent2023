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

    fn neighbor_direction(&self, x: usize, y: usize) -> Direction {
        if self.position[0] + 1 == x {
            return Direction::East;
        }
        if self.position[0] == x + 1 {
            return Direction::West;
        }
        if self.position[1] + 1 == y {
            return Direction::South;
        }
        if self.position[1] == y + 1 {
            return Direction::North;
        }
        Direction::Unknown
    }

    fn to_regular(&self, neighbors: &Vec<[usize; 2]>) -> Self {
        Pipe {
            position: self.position,
            endpoints: [
                self.neighbor_direction(neighbors[0][0], neighbors[0][1]),
                self.neighbor_direction(neighbors[1][0], neighbors[1][1]),
                Direction::Unknown,
                Direction::Unknown,
            ],
            start: self.start,
            ground: self.ground,
        }
    }
}

#[derive(Debug)]
struct Maze {
    contents: Array2<Pipe>,
    start: [usize; 2],
    path: Vec<Pipe>,
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
            path: Vec::new(),
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

        if x + 1 < self.contents.shape()[1] {
            ret.push([x + 1, y]);
        }

        if y + 1 < self.contents.shape()[0] {
            ret.push([x, y + 1]);
        }

        ret
    }

    fn path_len(&mut self) -> usize {
        let mut len = 1;

        let neighbors = self.get_neighbors(self.start[0], self.start[1]);
        let start: Pipe = *self.contents.get((self.start[1], self.start[0])).unwrap();
        let nexts: Vec<[usize; 2]> = neighbors
            .iter()
            .filter(|x| start.is_compatible(self.contents.get((x[1], x[0])).unwrap()))
            .cloned()
            .collect();
        self.contents[(start.position[1], start.position[0])] = start.to_regular(&nexts);

        let mut cur = start;
        let mut next: Pipe = *self.contents.get((nexts[0][1], nexts[0][0])).unwrap();

        self.path.push(start.clone());
        self.path.push(next.clone());

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
            self.path.push(next.clone());
            next = compatibles[0];
            len += 1;
        }

        len
    }

    fn enclosed_tiles(&self) -> usize {
        let mut ret = 0;
        for row in self.contents.rows() {
            let mut crossings = 0;
            let mut prev_corner = Direction::Unknown;

            for pipe in row {
                if self.path.contains(pipe) {
                    if pipe.endpoints.contains(&Direction::North)
                        && pipe.endpoints.contains(&Direction::South)
                    {
                        crossings += 1;
                    }
                    if pipe.endpoints.contains(&Direction::East) {
                        if pipe.endpoints.contains(&Direction::South) {
                            prev_corner = Direction::South;
                        }
                        if pipe.endpoints.contains(&Direction::North) {
                            prev_corner = Direction::North;
                        }
                    }
                    if pipe.endpoints.contains(&Direction::West)
                        && !pipe.endpoints.contains(&Direction::East)
                    {
                        if !pipe.endpoints.contains(&prev_corner) {
                            crossings += 1;
                        }
                    }
                } else if crossings % 2 != 0 {
                    ret += 1;
                }
            }
        }
        ret
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
    let mut maze = Maze::new(&contents);
    println!("{maze:?}");

    maze.path_len();

    let enclose = maze.enclosed_tiles();
    println!("{enclose:?}");
}
