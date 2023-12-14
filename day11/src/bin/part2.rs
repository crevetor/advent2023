use matrix::Matrix;
use std::cmp::{max, min};
use std::env;
use std::fs;
use std::process;

trait Universe {
    fn galaxy_positions(&self) -> Vec<[usize; 2]>;
    fn galaxy_pairs(&self) -> Vec<[[usize; 2]; 2]>;
}

impl Universe for Matrix<char> {
    fn galaxy_positions(&self) -> Vec<[usize; 2]> {
        let mut ret = Vec::new();
        for (y, row) in self.rows().enumerate() {
            for (x, elt) in row.iter().enumerate() {
                if elt == &'#' {
                    ret.push([x, y]);
                }
            }
        }

        ret
    }

    fn galaxy_pairs(&self) -> Vec<[[usize; 2]; 2]> {
        let mut ret = Vec::new();
        let pos = self.galaxy_positions();
        for first in &pos {
            for second in &pos {
                if first != second
                    && !ret.contains(&[*first, *second])
                    && !ret.contains(&[*second, *first])
                {
                    ret.push([*first, *second]);
                }
            }
        }

        ret
    }
}

fn distance(
    start: [usize; 2],
    end: [usize; 2],
    empty_cols: &Vec<usize>,
    empty_rows: &Vec<usize>,
) -> usize {
    let minx = min(start[0], end[0]);
    let maxx = max(start[0], end[0]);
    let miny = min(start[1], end[1]);
    let maxy = max(start[1], end[1]);
    let factor: isize = 1000000;

    let num_empty_cols = isize::try_from(
        empty_cols
            .iter()
            .filter(|x| x > &&minx && x < &&maxx)
            .count(),
    )
    .unwrap();
    let num_empty_rows = isize::try_from(
        empty_rows
            .iter()
            .filter(|x| x > &&miny && x < &&maxy)
            .count(),
    )
    .unwrap();

    let start = [
        isize::try_from(start[0]).unwrap(),
        isize::try_from(start[1]).unwrap(),
    ];
    let end = [
        isize::try_from(end[0]).unwrap(),
        isize::try_from(end[1]).unwrap(),
    ];
    usize::try_from(
        (end[0] - start[0]).abs() - num_empty_cols + (end[1] - start[1]).abs() - num_empty_rows
            + num_empty_cols * factor
            + num_empty_rows * factor,
    )
    .unwrap()
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
    Matrix::new(contents.iter().map(|x| x.chars().collect()).collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let matrix = parse_input(&contents);
    println!("{}", matrix);

    let mut empty_rows = Vec::new();
    for (i, row) in matrix.rows().enumerate() {
        if row.iter().all(|x| x == &'.') {
            empty_rows.push(i);
        }
    }

    let mut empty_cols = Vec::new();
    for (i, col) in matrix.cols().enumerate() {
        if col.iter().all(|x| x == &'.') {
            empty_cols.push(i);
        }
    }

    let pairs = matrix.galaxy_pairs();
    println!("{pairs:?}");
    println!("{}", pairs.len());
    let distances = pairs
        .iter()
        .map(|x| distance(x[0], x[1], &empty_cols, &empty_rows))
        .collect::<Vec<usize>>();
    println!("{distances:?}");
    println!("{}", distances.iter().sum::<usize>());
}
