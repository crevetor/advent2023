use std::env;
use std::fs;
use std::process;

fn forward(values: &Vec<isize>) -> Vec<isize> {
    let mut next = Vec::new();
    for i in 0..values.len() - 1 {
        next.push(values[i + 1] - values[i]);
    }
    if next.iter().all(|x| x == &0) {
        next.push(next[0]);
        return next;
    } else {
        let new = forward(&next);
        next.push(next.iter().last().unwrap() + new.iter().last().unwrap());
        return next;
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
    let mut histories: Vec<Vec<isize>> = contents
        .iter()
        .map(|x| x.split(' ').map(|z| z.parse().unwrap()).collect())
        .collect();

    for history in histories.iter_mut() {
        let vals = forward(&history);
        history.push(history[history.len() - 1] + vals.iter().last().unwrap());
    }
    println!("{:?}", histories);
    println!(
        "{}",
        histories
            .iter()
            .map(|x| x.iter().last().unwrap())
            .sum::<isize>()
    );
}
