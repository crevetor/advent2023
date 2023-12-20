use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Rule {
    field: Option<char>,
    cmp: Option<char>,
    val: Option<i64>,
    next: String,
}

#[derive(Debug)]
struct RuleErr;
impl FromStr for Rule {
    type Err = RuleErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|x| x.is_alphabetic()) {
            let (rule, next) = s.split_once(':').unwrap();
            Ok(Rule {
                field: Some(rule.chars().nth(0).unwrap()),
                cmp: Some(rule.chars().nth(1).unwrap()),
                val: Some(rule.chars().skip(2).collect::<String>().parse().unwrap()),
                next: next.to_string(),
            })
        } else {
            Ok(Rule {
                field: None,
                cmp: None,
                val: None,
                next: s.to_string(),
            })
        }
    }
}

impl Rule {
    fn process(&self, range: Range) -> (Range, Range) {
        let (mut acc, mut rej) = (range, range);
        match self.cmp {
            Some('<') => {
                acc.set_end(self.field.unwrap(), self.val.unwrap() - 1);
                rej.set_start(self.field.unwrap(), self.val.unwrap());
            }
            Some('>') => {
                acc.set_start(self.field.unwrap(), self.val.unwrap() + 1);
                rej.set_end(self.field.unwrap(), self.val.unwrap());
            }
            _ => panic!("This shouldn't happend"),
        }
        (acc, rej)
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    x: [i64; 2],
    m: [i64; 2],
    a: [i64; 2],
    s: [i64; 2],
}

impl Default for Range {
    fn default() -> Self {
        Self {
            x: [1, 4000],
            m: [1, 4000],
            a: [1, 4000],
            s: [1, 4000],
        }
    }
}

impl Range {
    fn set_start(&mut self, field: char, val: i64) {
        match field {
            'x' => self.x[0] = val,
            'm' => self.m[0] = val,
            'a' => self.a[0] = val,
            's' => self.s[0] = val,
            _ => panic!("This shouldn't happen"),
        }
    }

    fn set_end(&mut self, field: char, val: i64) {
        match field {
            'x' => self.x[1] = val,
            'm' => self.m[1] = val,
            'a' => self.a[1] = val,
            's' => self.s[1] = val,
            _ => panic!("This shouldn't happen"),
        }
    }

    fn combinations(&self) -> i64 {
        (self.x[1] - self.x[0] + 1)
            * (self.m[1] - self.m[0] + 1)
            * (self.a[1] - self.a[0] + 1)
            * (self.s[1] - self.s[0] + 1)
    }
}

#[derive(Debug)]
struct WorkflowErr;
impl FromStr for Workflow {
    type Err = WorkflowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rules) = s.split_once('{').unwrap();
        Ok(Workflow {
            id: id.to_string(),
            rules: rules
                .strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|x| Rule::from_str(x).unwrap())
                .collect(),
        })
    }
}

impl Workflow {
    fn process(&self, range: Range) -> Vec<(Range, String)> {
        let mut ret = Vec::new();
        let mut accept_range = range;
        let mut reject_range = range;
        for rule in self.rules.iter() {
            if rule.field.is_some() {
                (accept_range, reject_range) = rule.process(reject_range);
                ret.push((accept_range, rule.next.clone()));
            } else {
                ret.push((reject_range, rule.next.clone()));
            }
        }
        ret
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

    let workflows: Vec<Workflow> = contents
        .iter()
        .take_while(|x| !x.is_empty())
        .map(|x| Workflow::from_str(x).unwrap())
        .collect();
    println!("{workflows:?}");

    let workflows_map: HashMap<String, Workflow> = HashMap::from_iter(
        workflows
            .iter()
            .map(|x| x.id.clone())
            .zip(workflows.iter().cloned()),
    );
    println!("{workflows_map:?}");
    let mut ranges: Vec<(Range, String)> = vec![(Range::default(), "in".to_string())];
    while !ranges.iter().all(|x| x.1 == "A" || x.1 == "R") {
        let mut new_ranges = Vec::new();
        for (range, workflow) in ranges.iter().cloned().filter(|x| x.1 != "A" && x.1 != "R") {
            new_ranges.append(&mut workflows_map.get(&workflow).unwrap().process(range));
        }
        ranges = ranges
            .iter()
            .filter(|x| x.1 == "A" || x.1 == "R")
            .cloned()
            .collect();
        ranges.append(&mut new_ranges);
    }
    let accepted_ranges = ranges
        .iter()
        .filter(|x| x.1 == "A")
        .map(|x| x.0)
        .collect::<Vec<Range>>();

    for range in accepted_ranges.iter() {
        println!("{range:?}");
    }
    println!(
        "{}",
        accepted_ranges
            .iter()
            .map(|x| x.combinations())
            .sum::<i64>()
    );
}
