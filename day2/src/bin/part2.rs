use std::env;
use std::fs;
use std::process;

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

fn game_power(game: &Game) -> usize {
    let min_r = game
        .cubesets
        .iter()
        .map(|x| x.0)
        .max()
        .expect("Couldn't extract max R");
    let min_g = game
        .cubesets
        .iter()
        .map(|x| x.1)
        .max()
        .expect("Couldn't extract max G");
    let min_b = game
        .cubesets
        .iter()
        .map(|x| x.2)
        .max()
        .expect("Couldn't extract max B");
    return min_r * min_g * min_b;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args");
        process::exit(1);
    }

    let contents = read_input(&args[1]);
    let games = parse_games(&contents);
    let powers: Vec<usize> = games.iter().map(|x| game_power(x)).collect();
    println!("{}", powers.iter().sum::<usize>());
}
