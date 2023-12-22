use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Signal {
    from: String,
    to: String,
    pulse: bool,
}

impl Signal {
    fn new(from: &str, to: &str, pulse: bool) -> Self {
        Signal {
            from: from.to_string(),
            to: to.to_string(),
            pulse,
        }
    }
}

#[derive(Clone, Debug)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
    Button,
}

#[derive(Debug)]
struct ModduleTypeErr;
impl FromStr for ModuleType {
    type Err = ModduleTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (module, connections) = s.split_once(" -> ").unwrap();
        let connections: Vec<String> = connections.split(", ").map(|x| x.to_string()).collect();
        match module.chars().nth(0).unwrap() {
            '%' => Ok(ModuleType::FlipFlop(FlipFlop::new(
                module.chars().skip(1).collect::<String>(),
                connections.clone(),
            ))),
            '&' => Ok(ModuleType::Conjunction(Conjunction::new(
                module.chars().skip(1).collect::<String>(),
                connections.clone(),
            ))),
            _ => Ok(ModuleType::Broadcast(Broadcast::new(
                module.to_string(),
                connections.clone(),
            ))),
        }
    }
}

impl ModuleType {
    fn get_name(&self) -> String {
        match self {
            ModuleType::FlipFlop(f) => f.name.clone(),
            ModuleType::Conjunction(c) => c.name.clone(),
            ModuleType::Broadcast(b) => b.name.clone(),
            ModuleType::Button => "".to_string(),
        }
    }

    fn add_upstream(&mut self, name: String) {
        match self {
            ModuleType::FlipFlop(f) => f.upstream.push(name),
            ModuleType::Conjunction(c) => {
                c.upstream.push(name);
                c.last.push(false)
            }
            ModuleType::Broadcast(b) => b.upstream.push(name),
            ModuleType::Button => (),
        }
    }

    fn get_downstreams(&self) -> Vec<String> {
        match self {
            ModuleType::FlipFlop(f) => f.downstream.clone(),
            ModuleType::Conjunction(c) => c.downstream.clone(),
            ModuleType::Broadcast(b) => b.downstream.clone(),
            ModuleType::Button => vec![],
        }
    }

    fn run(&mut self, signal: &Signal) -> Vec<Signal> {
        match self {
            ModuleType::FlipFlop(f) => f.run(signal),
            ModuleType::Conjunction(c) => c.run(signal),
            ModuleType::Broadcast(b) => b.run(signal),
            ModuleType::Button => vec![],
        }
    }

    fn pulses(&self) -> [i64; 2] {
        match self {
            ModuleType::FlipFlop(f) => f.pulses,
            ModuleType::Conjunction(c) => c.pulses,
            ModuleType::Broadcast(b) => b.pulses,
            ModuleType::Button => [1, 0],
        }
    }
}

trait ModuleRun {
    fn run(&mut self, signal: &Signal) -> Vec<Signal>;
}

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    upstream: Vec<String>,
    downstream: Vec<String>,
    pulses: [i64; 2],
    state: bool,
}

impl FlipFlop {
    fn new(name: String, downstream: Vec<String>) -> Self {
        FlipFlop {
            name,
            upstream: Vec::new(),
            downstream,
            pulses: [0, 0],
            state: false,
        }
    }
}

impl ModuleRun for FlipFlop {
    fn run(&mut self, signal: &Signal) -> Vec<Signal> {
        if !signal.pulse {
            self.pulses[0] += 1;
            self.state = !self.state;
            self.downstream
                .iter()
                .map(|x| Signal::new(&self.name, x, self.state))
                .collect()
        } else {
            self.pulses[1] += 1;
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    name: String,
    upstream: Vec<String>,
    downstream: Vec<String>,
    pulses: [i64; 2],
    last: Vec<bool>,
}

impl Conjunction {
    fn new(name: String, downstream: Vec<String>) -> Self {
        Conjunction {
            name,
            upstream: Vec::new(),
            downstream,
            pulses: [0, 0],
            last: Vec::new(),
        }
    }
}

impl ModuleRun for Conjunction {
    fn run(&mut self, signal: &Signal) -> Vec<Signal> {
        if signal.pulse {
            self.pulses[1] += 1;
        } else {
            self.pulses[0] += 1;
        }

        let idx = self
            .upstream
            .iter()
            .position(|x| x == &signal.from)
            .unwrap();
        self.last[idx] = signal.pulse;
        if self.last.iter().all(|x| *x) {
            self.downstream
                .iter()
                .map(|x| Signal::new(&self.name, x, false))
                .collect()
        } else {
            self.downstream
                .iter()
                .map(|x| Signal::new(&self.name, x, true))
                .collect()
        }
    }
}

#[derive(Clone, Debug)]
struct Broadcast {
    name: String,
    upstream: Vec<String>,
    downstream: Vec<String>,
    pulses: [i64; 2],
}

impl Broadcast {
    fn new(name: String, downstream: Vec<String>) -> Self {
        Broadcast {
            name,
            upstream: Vec::new(),
            downstream,
            pulses: [0, 0],
        }
    }
}

impl ModuleRun for Broadcast {
    fn run(&mut self, signal: &Signal) -> Vec<Signal> {
        if signal.pulse {
            self.pulses[1] += 1;
        } else {
            self.pulses[0] += 1;
        }

        self.downstream
            .iter()
            .map(|x| Signal::new(&self.name, x, signal.pulse))
            .collect()
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
    let mut modules: HashMap<String, ModuleType> = HashMap::from_iter(contents.iter().map(|x| {
        let m = ModuleType::from_str(x).unwrap();
        (m.get_name(), m)
    }));
    for module in modules.clone().values() {
        for name in module.get_downstreams().iter().filter(|x| x != &"rx") {
            modules
                .get_mut(name)
                .unwrap()
                .add_upstream(module.get_name());
        }
    }
    println!("{modules:?}");

    let mut output_signals = [0, 0];
    for _ in 0..1000 {
        let mut next_signals: Vec<Signal> = vec![Signal::new("button", "broadcaster", false)];
        while !next_signals.is_empty() {
            println!("{next_signals:?}");
            let mut collected_signals = Vec::new();
            for signal in next_signals {
                if signal.to != "rx" {
                    collected_signals
                        .append(&mut modules.get_mut(&signal.to).unwrap().run(&signal));
                } else {
                    if signal.pulse {
                        output_signals[1] += 1;
                    } else {
                        output_signals[0] += 1;
                    }
                }
            }
            next_signals = collected_signals;
        }
        println!("--------------");
    }
    let pulses = modules
        .values()
        .map(|x| x.pulses())
        .collect::<Vec<[i64; 2]>>();

    println!("{pulses:?}");
    println!("{output_signals:?}");
    let mut pulses = pulses
        .iter()
        .fold([0, 0], |acc, x| [acc[0] + x[0], acc[1] + x[1]]);
    pulses[0] += output_signals[0];
    pulses[1] += output_signals[1];
    println!("{}", pulses[0] * pulses[1]);
}
