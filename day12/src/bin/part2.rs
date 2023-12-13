use std::convert::From;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
    Invalid,
}

impl From<char> for SpringState {
    fn from(c: char) -> SpringState {
        match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => SpringState::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<SpringState>,
    damaged: Vec<usize>,
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(' ').unwrap();
        Record {
            springs: left.chars().map(|c| SpringState::from(c)).collect(),
            damaged: right
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

impl Record {
    fn to_part2(&self) -> Record {
        let mut ret = self.clone();

        //TODO: figure out the rule on where to insert the '?'

        if self.springs[self.springs.len() - 1] == SpringState::Damaged {
            ret.springs.push(SpringState::Unknown);
        }
        if self.springs[self.springs.len() - 1] == SpringState::Operational {
            ret.springs.insert(0, SpringState::Unknown);
        }
        if self.springs[self.springs.len() - 1] == SpringState::Unknown {
            if self.damaged[self.damaged.len() - 1] == 1 {
                ret.springs.push(SpringState::Unknown);
            } else {
                ret.springs.insert(0, SpringState::Unknown);
            }
        }

        ret
    }

    fn damaged_pattern(&self, springs: &Vec<SpringState>) -> Vec<usize> {
        let mut ret = Vec::new();

        let mut damaged_count = 0;
        for spring in springs {
            match *spring {
                SpringState::Damaged => damaged_count += 1,
                SpringState::Operational => {
                    if damaged_count > 0 {
                        ret.push(damaged_count);
                        damaged_count = 0;
                    }
                }
                _ => panic!("Got an invalid springstate to count"),
            }
        }
        if damaged_count > 0 {
            ret.push(damaged_count);
        }

        ret
    }

    fn arrangements(&self) -> usize {
        self.walk(&Vec::new(), 0)
    }

    fn walk(&self, prev: &Vec<SpringState>, idx: usize) -> usize {
        let mut news = Vec::new();
        if self.springs[idx] != SpringState::Unknown {
            let mut new = prev.clone();
            new.push(self.springs[idx]);
            news.push(new);
        } else {
            for state in [SpringState::Damaged, SpringState::Operational] {
                let mut new = prev.clone();
                new.push(state);
                news.push(new);
            }
        }

        let mut sum = 0;
        for new in news {
            let pat = self.damaged_pattern(&new);
            if pat.len() == 0
                || (pat.len() <= self.damaged.len()
                    && pat[pat.len() - 1] <= self.damaged[pat.len() - 1])
            {
                if new.len() == self.springs.len() && pat == self.damaged {
                    sum += 1;
                } else if new.len() != self.springs.len() {
                    sum += self.walk(&new, idx + 1);
                }
            }
        }
        sum
    }
}

fn parse_input(contents: Vec<String>) -> Vec<Record> {
    contents.iter().map(|s| Record::from(s.as_str())).collect()
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
    let records: Vec<Record> = parse_input(contents);

    let arrangements = records
        .iter()
        .map(|r| r.arrangements())
        .collect::<Vec<usize>>();

    println!("{arrangements:?}");

    let newrecs: Vec<Record> = records.iter().map(|r| r.to_part2()).collect();
    println!("{newrecs:?}");
    let newarrangements = newrecs
        .iter()
        .map(|r| r.arrangements())
        .collect::<Vec<usize>>();
    println!("{newarrangements:?}");

    let mut totalarrangements = 0;
    for (arrangement, newarrangement) in arrangements.iter().zip(newarrangements.iter()) {
        totalarrangements += arrangement * newarrangement.pow(4);
    }
    println!("{}", totalarrangements);
}
