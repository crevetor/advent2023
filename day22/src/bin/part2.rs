use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    X = 0,
    Y,
    Z,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    idx: usize,
    start: [i32; 3],
    end: [i32; 3],
    orientation: Orientation,
    supports: Vec<usize>,
    supported: Vec<usize>,
}

#[derive(Debug)]
struct BrickParseError;
impl FromStr for Brick {
    type Err = BrickParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("~").unwrap();
        let start: [i32; 3] = start
            .split(",")
            .map(|x| i32::from_str_radix(x, 10).unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        let end: [i32; 3] = end
            .split(",")
            .map(|x| i32::from_str_radix(x, 10).unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        Ok(Brick::new(0, start, end))
    }
}

impl Brick {
    fn new(idx: usize, start: [i32; 3], end: [i32; 3]) -> Self {
        let orientation = if start[0] != end[0] {
            Orientation::X
        } else if start[1] != end[1] {
            Orientation::Y
        } else if start[2] != end[2] {
            Orientation::Z
        } else {
            Orientation::None
        };
        Brick {
            idx,
            start,
            end,
            orientation,
            supports: Vec::new(),
            supported: Vec::new(),
        }
    }

    fn supports(&self, other: &Self) -> bool {
        if self.end[2] + 1 != other.start[2] {
            return false;
        }

        self.start[0] <= other.end[0]
            && self.end[0] >= other.start[0]
            && self.start[1] <= other.end[1]
            && self.end[1] >= other.start[1]
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
    let mut bricks: Vec<Brick> = contents
        .iter()
        .map(|x| Brick::from_str(x).unwrap())
        .collect();
    bricks.sort_by_key(|x| x.start[2]);
    bricks
        .iter_mut()
        .enumerate()
        .for_each(|(idx, brick)| brick.idx = idx);
    let mut newbricks: HashMap<usize, Brick> = HashMap::new();
    for brick in bricks.iter_mut() {
        while brick.start[2] > 1 && !newbricks.values().any(|x| x.supports(brick)) {
            brick.start[2] -= 1;
            brick.end[2] -= 1;
        }
        let support_idx: Vec<usize> = newbricks
            .values_mut()
            .filter(|x| x.supports(brick))
            .map(|x| {
                x.supports.push(brick.idx);
                x.idx
            })
            .collect();
        let mut newbrick = brick.clone();
        newbrick.supported = support_idx.clone();
        newbricks.insert(newbrick.idx, newbrick);
    }
    println!("{newbricks:?}");

    let mut wouldfall: Vec<usize> = Vec::new();
    for brick in newbricks.values() {
        let mut deleted: HashSet<usize> = HashSet::new();
        deleted.insert(brick.idx);
        let mut cur: HashSet<Brick> = brick
            .supports
            .iter()
            .map(|x| newbricks.get(x).unwrap())
            .cloned()
            .collect();
        while !cur.is_empty() {
            let mut newcur: HashSet<Brick> = HashSet::new();
            for b in cur.iter() {
                if b.supported.iter().all(|x| deleted.contains(x)) {
                    deleted.insert(b.idx);
                    newcur.extend(
                        &mut b
                            .supports
                            .iter()
                            .map(|x| newbricks.get(x).unwrap())
                            .cloned(),
                    );
                }
            }
            cur = newcur;
        }
        deleted.remove(&brick.idx);
        wouldfall.append(&mut deleted.iter().cloned().collect());
    }
    println!("{}", wouldfall.len());
}
