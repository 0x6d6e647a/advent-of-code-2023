use std::collections::HashMap;
use std::io::{stdin, BufRead, Stdin};

// -----------------------------------------------------------------------------
#[derive(Clone)]
struct InclusiveRange {
    begin: usize,
    end: usize,
}

impl InclusiveRange {
    fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    fn len(&self) -> usize {
        self.end - self.begin + 1
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

use Category::*;

const CATEGORIES: [Category; 4] = [X, M, A, S];

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
    fn split_range(&self, range: &InclusiveRange) -> (InclusiveRange, InclusiveRange) {
        match self.cmp_op {
            GreaterThan => (
                InclusiveRange::new(self.value + 1, range.end),
                InclusiveRange::new(range.begin, self.value),
            ),
            LessThan => (
                InclusiveRange::new(range.begin, self.value - 1),
                InclusiveRange::new(self.value, range.end),
            ),
        }
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

// -----------------------------------------------------------------------------
struct System {
    workflows: HashMap<String, Workflow>,
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

        Self { workflows }
    }
}

impl System {
    fn combinations(&self, mut ranges: HashMap<Category, InclusiveRange>, dst: String) -> usize {
        // -- Result settled, calculate possibilities.
        if dst == "R" {
            return 0;
        } else if dst == "A" {
            return ranges.values().map(InclusiveRange::len).product();
        }

        // -- Continue calculating combinations.
        let mut total = 0;
        let mut do_fallback = true;

        let workflow = self.workflows.get(&dst).unwrap();

        for rule in workflow.rules.iter() {
            let range = ranges.get(&rule.category).unwrap();
            let (true_range, false_range) = rule.split_range(range);

            if true_range.begin <= true_range.end {
                let mut new_ranges = ranges.clone();
                new_ranges.insert(rule.category, true_range);
                total += self.combinations(new_ranges, rule.dst.clone());
            }

            if false_range.begin <= false_range.end {
                ranges.insert(rule.category, false_range);
            } else {
                do_fallback = false;
                break;
            }
        }

        if do_fallback {
            total += self.combinations(ranges, workflow.dst.clone());
        }

        total
    }
}

// -----------------------------------------------------------------------------
fn main() {
    let system = System::from(&stdin());
    let ranges: HashMap<Category, InclusiveRange> = CATEGORIES
        .iter()
        .map(|category| (*category, InclusiveRange::new(1, 4000)))
        .collect();
    let combinations = system.combinations(ranges, String::from("in"));
    println!("{}", combinations);
}
