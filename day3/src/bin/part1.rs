use std::env;
use std::fs;
use std::process;

use std::str::FromStr;

enum EltType {
    Symbol(char),
    Number(usize),
}

struct Elt {
    val: EltType,
    coords: Vec<(usize, usize)>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseEltErr;

impl FromStr for Elt {
    type Err = ParseEltErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

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
}