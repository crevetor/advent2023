use matrix::Matrix;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    Right,
    Left,
    Invalid,
}

impl From<char> for MirrorType {
    fn from(value: char) -> Self {
        match value {
            '/' => MirrorType::Right,
            '\\' => MirrorType::Left,
            _ => MirrorType::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SplitterType {
    Vertical,
    Horizontal,
    Invalid,
}

impl From<char> for SplitterType {
    fn from(value: char) -> Self {
        match value {
            '|' => SplitterType::Vertical,
            '-' => SplitterType::Horizontal,
            _ => SplitterType::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CellType {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
    Invalid,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            '|' | '-' => CellType::Splitter(SplitterType::from(value)),
            '/' | '\\' => CellType::Mirror(MirrorType::from(value)),
            '.' => CellType::Empty,
            _ => CellType::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ContraptionCell {
    celltype: CellType,
    visited: bool,
}

impl From<char> for ContraptionCell {
    fn from(value: char) -> Self {
        ContraptionCell {
            celltype: CellType::from(value),
            visited: false,
        }
    }
}

trait Visited {
    fn visited(&mut self, x: usize, y: usize);
    fn count_visited(&self) -> usize;
}

impl Visited for Matrix<ContraptionCell> {
    fn visited(&mut self, x: usize, y: usize) {
        self.get_mut(x, y).unwrap().visited = true;
    }

    fn count_visited(&self) -> usize {
        self.rows()
            .fold(0, |acc, x| acc + x.iter().filter(|c| c.visited).count())
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    pos: [i32; 2],
    velocity: [i32; 2],
    boundaries: [i32; 2],
    stopped: bool,
}

impl Beam {
    fn new(pos: [usize; 2], velocity: [i32; 2], boundaries: [usize; 2]) -> Self {
        Beam {
            pos: [pos[0].try_into().unwrap(), pos[1].try_into().unwrap()],
            velocity,
            boundaries: [
                boundaries[0].try_into().unwrap(),
                boundaries[1].try_into().unwrap(),
            ],
            stopped: false,
        }
    }

    fn step(&mut self) {
        if self.stopped {
            return;
        }

        let newx = self.pos[0] + self.velocity[0];
        let newy = self.pos[1] + self.velocity[1];
        if newx >= self.boundaries[0] || newx < 0 || newy >= self.boundaries[1] || newy < 0 {
            self.stopped = true;
        } else {
            self.pos[0] = newx;
            self.pos[1] = newy;
        }
    }

    fn encounter_mirror(&mut self, mirror: MirrorType) {
        match mirror {
            MirrorType::Right => {
                if self.velocity[0] > 0 {
                    self.velocity[0] = 0;
                    self.velocity[1] = -1;
                } else if self.velocity[0] < 0 {
                    self.velocity[0] = 0;
                    self.velocity[1] = 1;
                } else if self.velocity[1] > 0 {
                    self.velocity[0] = -1;
                    self.velocity[1] = 0;
                } else if self.velocity[1] < 0 {
                    self.velocity[0] = 1;
                    self.velocity[1] = 0;
                }
            }
            MirrorType::Left => {
                if self.velocity[0] > 0 {
                    self.velocity[0] = 0;
                    self.velocity[1] = 1;
                } else if self.velocity[0] < 0 {
                    self.velocity[0] = 0;
                    self.velocity[1] = -1;
                } else if self.velocity[1] > 0 {
                    self.velocity[0] = 1;
                    self.velocity[1] = 0;
                } else if self.velocity[1] < 0 {
                    self.velocity[0] = -1;
                    self.velocity[1] = 0;
                }
            }
            MirrorType::Invalid => panic!("Got an invalid mirror type"),
        }
    }

    fn encounter_splitter(&mut self, splitter: SplitterType) -> Option<Beam> {
        match splitter {
            SplitterType::Vertical => {
                if self.velocity[0] != 0 {
                    self.velocity[0] = 0;
                    self.velocity[1] = -1;
                    let mut newbeam = self.clone();
                    newbeam.velocity[0] = 0;
                    newbeam.velocity[1] = 1;
                    Some(newbeam)
                } else {
                    None
                }
            }
            SplitterType::Horizontal => {
                if self.velocity[1] != 0 {
                    self.velocity[0] = -1;
                    self.velocity[1] = 0;
                    let mut newbeam = self.clone();
                    newbeam.velocity[0] = 1;
                    newbeam.velocity[1] = 0;
                    Some(newbeam)
                } else {
                    None
                }
            }
            SplitterType::Invalid => panic!("Found invalid splitter type"),
        }
    }

    fn interact(&mut self, cell: &mut ContraptionCell) -> Option<Beam> {
        cell.visited = true;
        match cell.celltype {
            CellType::Empty => None,
            CellType::Mirror(mirrortype) => {
                self.encounter_mirror(mirrortype);
                None
            }
            CellType::Splitter(splittertype) => self.encounter_splitter(splittertype),
            CellType::Invalid => panic!("Encountered invalid Cell"),
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

fn parse_input(contents: &Vec<String>) -> Matrix<ContraptionCell> {
    Matrix::from_iter(
        contents
            .iter()
            .map(|row| row.chars().map(|c| ContraptionCell::from(c)).collect()),
    )
}

fn energize(
    contraption: &mut Matrix<ContraptionCell>,
    start: [usize; 2],
    velocity: [i32; 2],
) -> usize {
    // TODO: make beam
    let mut beams = vec![Beam::new(
        start,
        velocity,
        [contraption.num_cols(), contraption.num_rows()],
    )];
    let mut energized: Vec<usize> = Vec::new();

    while !beams.iter().all(|x| x.stopped) {
        let mut newbeams = Vec::new();
        for beam in beams.iter_mut().filter(|b| !b.stopped) {
            if let Some(b) = beam.interact(
                contraption
                    .get_mut(
                        beam.pos[0].try_into().unwrap(),
                        beam.pos[1].try_into().unwrap(),
                    )
                    .unwrap(),
            ) {
                newbeams.push(b);
            }
        }
        beams.append(&mut newbeams);
        beams
            .iter_mut()
            .filter(|b| !b.stopped)
            .for_each(|b| b.step());

        let visited = contraption.count_visited();
        if energized
            .iter()
            .rev()
            .take_while(|x| x == &&visited)
            .count()
            > 3
        {
            break;
        }
        energized.push(visited);
    }

    /*for row in contraption.rows() {
        println!(
            "{}",
            row.iter()
                .map(|c| if c.visited { '#' } else { '.' })
                .collect::<String>()
        );
    }*/
    contraption.count_visited()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let contraption = parse_input(&contents);
    let mut energies = Vec::new();
    for x in 0..contraption.num_cols() {
        energies.push(energize(&mut contraption.clone(), [x, 0], [0, 1]));
        energies.push(energize(
            &mut contraption.clone(),
            [x, contraption.num_cols() - 1],
            [0, -1],
        ))
    }

    for y in 0..contraption.num_rows() {
        energies.push(energize(&mut contraption.clone(), [0, y], [1, 0]));
        energies.push(energize(
            &mut contraption.clone(),
            [contraption.num_cols() - 1, y],
            [-1, 0],
        ));
    }

    println!("{}", energies.iter().max().unwrap());
}
