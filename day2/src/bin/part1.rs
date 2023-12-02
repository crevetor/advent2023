use std::env;
use std::fs;
use std::process;

const DEFAULT_BAG: (usize, usize, usize) = (12, 13, 14);

#[derive(Debug)]
struct Game {
    id: usize,
    cubesets: Vec<(usize, usize, usize)>,
}

fn read_input(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Unable to read from file.");
    for line in content.lines() {
        ret.push(line.trim().to_string());
    }
    ret
}

fn parse_games(contents: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for line in contents {
        let game_id: usize = line
            .split(':')
            .nth(0)
            .expect("Couldn't split on :")
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let mut game: Game = Game {
            id: game_id,
            cubesets: Vec::new(),
        };
        for sets in line.split(':').last().expect("No cubesets").split(";") {
            let mut cubeset: (usize, usize, usize) = (0, 0, 0);
            for color in sets.split(",") {
                let val = color
                    .chars()
                    .filter(|x| x.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .expect("No digits found");
                if color.trim().ends_with("red") {
                    cubeset.0 = val;
                } else if color.trim().ends_with("green") {
                    cubeset.1 = val;
                } else {
                    cubeset.2 = val;
                }
            }
            game.cubesets.push(cubeset);
        }
        games.push(game);
    }
    println!("{:?}", games);
    games
}

fn game_possible(game: &Game) -> bool {
    for cubeset in &game.cubesets {
        if cubeset.0 > DEFAULT_BAG.0 || cubeset.1 > DEFAULT_BAG.1 || cubeset.2 > DEFAULT_BAG.2 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let games = parse_games(&contents);
    let possible_ids: Vec<usize> = games
        .iter()
        .filter(|x| game_possible(x))
        .map(|x| x.id)
        .collect();
    println!("{:?}", possible_ids);
    println!("{}", possible_ids.iter().sum::<usize>());
}
