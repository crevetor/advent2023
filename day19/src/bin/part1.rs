use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter;
use std::process;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
#[derive(Debug)]
struct PartErr;
impl FromStr for Part {
    type Err = PartErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",");
        let x = vals
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let m = vals
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let a = vals
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let s = vals
            .next()
            .unwrap()
            .split('=')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Ok(Part { x, m, a, s })
    }
}

impl Part {
    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Rule {
    field: Option<char>,
    cmp: Option<char>,
    val: Option<i32>,
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
    fn matches(&self, p: &Part) -> bool {
        if self.field == None {
            return true;
        } else {
            let val = match self.field {
                Some('x') => p.x,
                Some('m') => p.m,
                Some('a') => p.a,
                Some('s') => p.s,
                _ => panic!("This shouldn't happen"),
            };
            match self.cmp {
                Some('<') => return val < self.val.unwrap(),
                Some('>') => return val > self.val.unwrap(),
                _ => panic!("This shouldn't happen"),
            };
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
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
    fn process(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.next.clone();
            }
        }
        panic!("This shouldn't happen")
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

    let parts: Vec<Part> = contents
        .iter()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .map(|x| Part::from_str(x).unwrap())
        .collect();
    println!("{parts:?}");

    let mut parts_workflow: HashMap<Part, String> =
        HashMap::from_iter(parts.iter().cloned().zip(iter::repeat("in".to_string())));
    println!("{parts_workflow:?}");
    let workflows_map: HashMap<String, Workflow> = HashMap::from_iter(
        workflows
            .iter()
            .map(|x| x.id.clone())
            .zip(workflows.iter().cloned()),
    );
    println!("{workflows_map:?}");

    while !parts_workflow.values().all(|x| x == "A" || x == "R") {
        for (part, next_wf) in parts_workflow
            .iter_mut()
            .filter(|x| x.1 != "A" && x.1 != "R")
        {
            let workflow = workflows_map.get(next_wf).unwrap();
            *next_wf = workflow.process(part);
        }
    }
    println!("{parts_workflow:?}");

    println!(
        "{}",
        parts_workflow
            .iter()
            .filter(|x| x.1 == "A")
            .map(|x| x.0.sum())
            .sum::<i32>()
    );
}
