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

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let mut digits: Vec<i64> = Vec::new();
    for line in contents {
        let line_digits = line.chars().filter(|x| x.is_digit(10));
        digits.push(
            format!("{}{}", line_digits.clone().nth(0).unwrap(), line_digits.last().unwrap())
                .parse()
                .unwrap()
        );
    }
    println!("{:?}", digits);
    println!("{}", digits.iter().sum::<i64>());
    Ok(())
}
