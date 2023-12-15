use std::env;
use std::fs;
use std::process;

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn hash(s: &str) -> u8 {
    let mut ret: u8 = 0;
    for c in s.bytes() {
        ret = ret.wrapping_add(c);
        ret = ret.wrapping_mul(17);
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
    let hashes = contents[0].split(',').map(|x| hash(x)).collect::<Vec<u8>>();
    println!("{hashes:?}");
    println!("{}", hashes.iter().map(|x| usize::from(*x)).sum::<usize>());
}
