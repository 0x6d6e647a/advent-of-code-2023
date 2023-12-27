use std::collections::{HashMap, VecDeque};
use std::io::{stdin, BufRead, Stdin};
use std::ops::Neg;
use std::rc::Rc;

// -----------------------------------------------------------------------------
fn gcd(a: usize, b: usize) -> usize {
    let mut m = a;
    let mut n = b;

    if m == 0 || n == 0 {
        return m | n;
    }

    let shift = (m | n).trailing_zeros();
    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }

    m << shift
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

use Pulse::*;

impl Neg for Pulse {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Low => High,
            High => Low,
        }
    }
}

// -----------------------------------------------------------------------------
struct Node {
    src: Rc<String>,
    pulse: Pulse,
    dst: Rc<String>,
}

impl Node {
    fn new(src: &Rc<String>, pulse: Pulse, dst: &Rc<String>) -> Self {
        let src = src.clone();
        let dst = dst.clone();
        Self { src, pulse, dst }
    }

    fn send_pulse(&self, modules: &mut HashMap<Rc<String>, Module>, queue: &mut VecDeque<Node>) {
        if let Some(module) = modules.get_mut(&self.dst) {
            for next_node in module.propagate_pulse(self.pulse, &self.src) {
                queue.push_back(next_node);
            }
        }
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

use ModuleType::*;

// -----------------------------------------------------------------------------
const MODULE_BUTTON_NAME: &str = "button";
const MODULE_BROADCAST_NAME: &str = "broadcaster";
const MODULE_FLIPFLOP_SIGIL: char = '%';
const MODULE_CONJUNCTION_SIGIL: char = '&';

#[derive(Clone)]
struct Module {
    name: Rc<String>,
    dsts: Vec<Rc<String>>,
    mtype: ModuleType,
    switch: bool,
    mem: HashMap<Rc<String>, Pulse>,
}

impl From<String> for Module {
    fn from(line: String) -> Self {
        let mut components = line.split_whitespace();
        let mut name = String::from(components.next().unwrap());

        // -- Determine switch type.
        let mtype = if name == MODULE_BROADCAST_NAME {
            Broadcast
        } else if name.starts_with(MODULE_FLIPFLOP_SIGIL) {
            name = String::from(&name[1..]);
            FlipFlop
        } else if name.starts_with(MODULE_CONJUNCTION_SIGIL) {
            name = String::from(&name[1..]);
            Conjunction
        } else {
            panic!("unable to determine module type for '{}'", name)
        };

        // -- Skip arrow.
        assert_eq!(components.next().unwrap().trim(), "->");

        // -- Parse destinations.
        let mut dsts = Vec::new();

        for dst in components {
            let dst = String::from(dst.replace(',', "").trim());
            dsts.push(dst.into());
        }

        // -- Construct module.
        let name = name.into();
        let switch = false;
        let mem = HashMap::new();

        Self {
            name,
            dsts,
            mtype,
            switch,
            mem,
        }
    }
}

impl Module {
    fn propagate_pulse(&mut self, receive: Pulse, from: &Rc<String>) -> Vec<Node> {
        let mut dsts = Vec::new();

        match self.mtype {
            Broadcast => {
                for dst in self.dsts.iter() {
                    let dst = Node::new(&self.name, receive, dst);
                    dsts.push(dst);
                }
            }
            FlipFlop => {
                if receive == Low {
                    let send = if self.switch { Low } else { High };

                    for dst in self.dsts.iter() {
                        let dst = Node::new(&self.name, send, dst);
                        dsts.push(dst);
                    }

                    self.switch ^= true;
                }
            }
            Conjunction => {
                self.mem.insert(from.clone(), receive);

                let send = if self.mem.values().all(|p| *p == High) {
                    Low
                } else {
                    High
                };

                for dst in self.dsts.iter() {
                    let dst = Node::new(&self.name, send, dst);
                    dsts.push(dst);
                }
            }
        }

        dsts
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone)]
struct System {
    modules: Vec<Module>,
}

impl From<&Stdin> for System {
    fn from(stdin: &Stdin) -> Self {
        let modules: Vec<_> = stdin
            .lock()
            .lines()
            .map(|line| Module::from(line.unwrap()))
            .collect();

        // -- Initalize memory for conjunction modules.
        let mut system = Self { modules };

        let mut conj_module_name_to_inputs = HashMap::new();

        for conj_module in system.modules.iter().filter(|module| module.mtype == Conjunction) {
            conj_module_name_to_inputs.insert(
                conj_module.name.clone(),
                system.find_module_inputs(&conj_module.name),
            );
        }

        for module in system.modules.iter_mut() {
            if let Some(inputs) = conj_module_name_to_inputs.get(&module.name) {
                for input in inputs {
                    module.mem.insert(input.clone(), Low);
                }
            }
        }

        system
    }
}

impl System {
    fn find_module_inputs(&self, name: &Rc<String>) -> Vec<Rc<String>> {
        let mut inputs = Vec::new();

        for modue in self.modules.iter() {
            if modue.dsts.contains(name) {
                inputs.push(modue.name.clone());
            }
        }

        inputs
    }

    fn init_module_map(&self) -> HashMap<Rc<String>, Module> {
        self.modules
            .iter()
            .map(|module| (module.name.clone(), (*module).clone()))
            .collect()
    }

    fn init_queue() -> VecDeque<Node> {
        let start = Node::new(
            &String::from(MODULE_BUTTON_NAME).into(),
            Low,
            &String::from(MODULE_BROADCAST_NAME).into(),
        );
        [start].into()
    }

    fn find_cycles(
        &self,
        target: &Rc<String>,
        inputs: &Vec<Rc<String>>,
    ) -> HashMap<Rc<String>, usize> {
        let mut modules = self.init_module_map();
        let mut cycle_tracker = HashMap::new();

        for num_button_presses in 1..usize::MAX {
            let mut queue = Self::init_queue();

            while let Some(curr_node) = queue.pop_front() {
                // -- Track cycles.
                if curr_node.pulse == High
                    && curr_node.dst == *target
                    && inputs.contains(&curr_node.src)
                    && !cycle_tracker.contains_key(&curr_node.src)
                {
                    cycle_tracker.insert(curr_node.src.clone(), num_button_presses);

                    if cycle_tracker.len() == inputs.len() {
                        return cycle_tracker;
                    }
                }

                // -- Send pulse.
                curr_node.send_pulse(&mut modules, &mut queue);
            }
        }

        panic!("no cycles found!");
    }

    fn find_required_button_presses(&self, name: &str) -> usize {
        let name = Rc::new(String::from(name));

        // -- Find conjunction that signals to rx.
        let final_conjunction = self.find_module_inputs(&name).first().unwrap().clone();

        // -- Find all inputs of that conjunction.
        let final_inputs = self.find_module_inputs(&final_conjunction);

        // -- Find where those inputs cycle.
        let cycle_map = self.find_cycles(&final_conjunction, &final_inputs);

        // -- Calculate when final module will be activated
        let mut required_button_presses = 1;

        for value in cycle_map.values() {
            required_button_presses = lcm(required_button_presses, *value);
        }

        required_button_presses
    }
}

// -----------------------------------------------------------------------------
fn main() {
    let system = System::from(&stdin());
    let num = system.find_required_button_presses("rx");
    println!("{}", num);
}
