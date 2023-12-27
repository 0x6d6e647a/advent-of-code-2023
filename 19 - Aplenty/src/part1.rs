use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

use Category::*;

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => X,
            'm' => M,
            'a' => A,
            's' => S,
            _ => panic!("invalid char for category: '{}'", value),
        }
    }
}

// -----------------------------------------------------------------------------
enum CmpOp {
    GreaterThan,
    LessThan,
}

use CmpOp::*;

impl From<char> for CmpOp {
    fn from(value: char) -> Self {
        match value {
            '<' => LessThan,
            '>' => GreaterThan,
            _ => panic!("invalid char for comparison operator: '{}'", value),
        }
    }
}

// -----------------------------------------------------------------------------
struct Rule {
    category: Category,
    cmp_op: CmpOp,
    value: usize,
    dst: String,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let category = value.chars().next().unwrap().into();
        let cmp_op = value.chars().nth(1).unwrap().into();
        let components = &value[2..];
        let mut components = components.split(':');
        let value = components.next().unwrap().parse().unwrap();
        let dst = components.next().unwrap().into();
        Self {
            category,
            cmp_op,
            value,
            dst,
        }
    }
}

impl Rule {
    fn process(&self, part: &Part) -> Option<String> {
        let part_value = part.get(&self.category);
        if match self.cmp_op {
            GreaterThan => part_value > self.value,
            LessThan => part_value < self.value,
        } {
            return Some(self.dst.clone());
        }

        None
    }
}

// -----------------------------------------------------------------------------
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    dst: String,
}

impl From<String> for Workflow {
    fn from(value: String) -> Self {
        let mut components = value.split('{');
        let name = components.next().unwrap().into();
        let rules_str = String::from(components.next().unwrap());
        let rules_str = rules_str.replace('}', "");
        let mut rule_strs: Vec<_> = rules_str.split(',').collect();
        let dst = String::from(rule_strs.pop().unwrap());

        let mut rules = Vec::new();

        for rule_str in rule_strs.into_iter() {
            rules.push(rule_str.into());
        }

        Self { name, rules, dst }
    }
}

impl Workflow {
    fn process(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            if let Some(dst) = rule.process(part) {
                return dst;
            }
        }

        self.dst.clone()
    }
}

// -----------------------------------------------------------------------------
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<String> for Part {
    fn from(value: String) -> Self {
        let mut values = value
            .split(['{', '=', ',', '}', 'x', 'm', 'a', 's'])
            .filter(|value| !value.is_empty());
        let x = values.next().unwrap().parse().unwrap();
        let m = values.next().unwrap().parse().unwrap();
        let a = values.next().unwrap().parse().unwrap();
        let s = values.next().unwrap().parse().unwrap();
        Self { x, m, a, s }
    }
}

impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl Part {
    fn get(&self, category: &Category) -> usize {
        match category {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }
}

// -----------------------------------------------------------------------------
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<&Stdin> for System {
    fn from(stdin: &Stdin) -> Self {
        let mut lines = stdin.lock().lines();

        // -- Parse workflows.
        let mut workflows: HashMap<String, Workflow> = HashMap::new();

        for line in lines.by_ref() {
            let line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let workflow: Workflow = line.into();
            let name = workflow.name.clone();
            workflows.insert(name, workflow);
        }

        // -- Parse parts.
        let mut parts = Vec::new();

        for line in lines.by_ref() {
            let line = line.unwrap();
            parts.push(line.into());
        }

        Self { workflows, parts }
    }
}

impl System {
    fn process(&self) -> usize {
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();

        for part in self.parts.iter() {
            let mut curr_workflow_name = String::from("in");

            while let Some(workflow) = self.workflows.get(&curr_workflow_name) {
                let dst = workflow.process(part);

                if dst == "A" {
                    accepted.push(part);
                } else if dst == "R" {
                    rejected.push(part);
                }

                curr_workflow_name = dst;
            }
        }

        accepted.iter().map(|part| part.value()).sum()
    }
}

// -----------------------------------------------------------------------------
fn main() {
    let system = System::from(&stdin());
    let sum = system.process();
    println!("{}", sum);
}
