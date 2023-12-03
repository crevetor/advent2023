use std::env;
use std::fs;
use std::process;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }

    ret
}

fn parse_line(line: &str) -> i64 {
    let mut my_line: Vec<char> = line.clone().chars().collect();
    for (i, num) in NUMBERS.iter().enumerate() {
        for idx in line.match_indices(num) {
            my_line[idx.0] = char::from_digit((i + 1).try_into().unwrap(), 10).unwrap();
        }
    }

    let line_digits = my_line.iter().filter(|x| x.is_digit(10));
    println!("{:?}", line_digits);
    format!(
        "{}{}",
        line_digits.clone().nth(0).unwrap(),
        line_digits.last().unwrap()
    )
    .parse()
    .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let digits: Vec<i64> = contents.iter().map(|line| parse_line(&line)).collect();
    println!("{:?}", digits);
    println!("{}", digits.iter().sum::<i64>());
}
