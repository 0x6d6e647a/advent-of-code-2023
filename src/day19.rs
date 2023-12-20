use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

// -- Inclusive Range.
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

// -- Category.
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

// -- Comparison Operator.
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

// -- Rule.
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

// -- Workflow.
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    dst: String,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
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

// -- Part.
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
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

// -- System.
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        // -- Parse workflows.
        let mut workflows: HashMap<String, Workflow> = HashMap::new();

        for line in lines.by_ref() {
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

#[aoc_generator(day19)]
fn parse(input: &str) -> System {
    input.into()
}

#[aoc(day19, part1)]
fn part1(system: &System) -> usize {
    system.process()
}

#[aoc(day19, part2)]
fn part2(system: &System) -> usize {
    let ranges: HashMap<Category, InclusiveRange> = CATEGORIES
        .iter()
        .map(|category| (*category, InclusiveRange::new(1, 4000)))
        .collect();
    system.combinations(ranges, String::from("in"))
}
