use std::env;
use std::fmt;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
struct Step {
    label: String,
    operation: char,
    lens: Option<i32>,
}

#[derive(Debug)]
struct StepErr;
impl FromStr for Step {
    type Err = StepErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let label = s
            .chars()
            .take_while(|x| x.is_alphabetic())
            .collect::<String>();
        let operation = s
            .chars()
            .skip_while(|x| x.is_alphabetic())
            .next()
            .unwrap()
            .clone();
        let lens: Option<i32> = match s.split_once(operation).unwrap().1.parse::<i32>() {
            Ok(r) => Some(r),
            Err(_) => None,
        };
        Ok(Step {
            label: label.to_string(),
            operation: operation,
            lens,
        })
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_len: i32,
}

impl Lens {
    fn new(label: &String, focal_len: i32) -> Self {
        Lens {
            label: label.clone(),
            focal_len,
        }
    }
}

#[derive(Clone, Debug)]
struct Box {
    id: usize,
    lenses: Vec<Option<Lens>>,
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box {}: {}",
            self.id,
            self.lenses
                .iter()
                .map(|x| match x {
                    Some(lens) => format!("[{} {}]", lens.label, lens.focal_len),
                    None => "".to_string(),
                })
                .collect::<String>(),
        )
    }
}

impl Box {
    fn new(id: usize) -> Self {
        Box {
            id,
            lenses: Vec::new(),
        }
    }

    fn lens_idx(&self, label: &String) -> Option<usize> {
        self.lenses.iter().position(|x| match x {
            Some(lens) => &lens.label == label,
            None => false,
        })
    }

    fn remove_lens(&mut self, label: &String) {
        let lens_pos = self.lens_idx(label);
        if let Some(idx) = lens_pos {
            for i in (0..idx).rev() {
                self.lenses[i + 1] = self.lenses[i].clone();
            }
            self.lenses[0] = None;
        }
    }

    fn add_lens(&mut self, lens: Lens) {
        let lens_pos = self.lens_idx(&lens.label);
        if let Some(idx) = lens_pos {
            self.lenses[idx] = Some(lens.clone());
        } else {
            self.lenses.push(Some(lens.clone()));
        }
    }

    fn power(&self) -> i32 {
        let mut sum = 0;
        let mut i = 1;
        for lens in self.lenses.iter() {
            if let Some(l) = lens {
                sum += (i32::try_from(self.id).unwrap() + 1)
                    * (i32::try_from(i).unwrap())
                    * l.focal_len;
                i += 1;
            }
        }
        sum
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
    let steps: Vec<Step> = contents[0]
        .split(",")
        .map(|x| Step::from_str(x).unwrap())
        .collect();
    let mut boxes = (0..256).map(|x| Box::new(x)).collect::<Vec<_>>();

    println!("{steps:?}");
    for step in steps {
        let box_idx = hash(&step.label);
        match step.operation {
            '-' => boxes[usize::from(box_idx)].remove_lens(&step.label),
            '=' => boxes[usize::from(box_idx)].add_lens(Lens::new(&step.label, step.lens.unwrap())),
            _ => (),
        }
    }

    let powers = boxes.iter().map(|x| x.power());
    println!("{:?}", powers.clone().collect::<Vec<i32>>());
    println!("{}", powers.sum::<i32>());
}
