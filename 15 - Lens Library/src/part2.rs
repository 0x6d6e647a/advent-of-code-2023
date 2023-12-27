use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::rc::Rc;

fn holiday_hash(input: &str) -> usize {
    let mut value = 0;

    for char in input.chars() {
        value += char as usize;
        value *= 17;
        value %= 256;
    }

    value
}

enum Command {
    Put {
        label: Rc<String>,
        focal: usize,
        holiday_hash: usize,
    },
    Get {
        label: Rc<String>,
        holiday_hash: usize,
    },
}

use Command::*;

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        if input.contains('=') {
            let mut components = input.split('=');
            let label = components.next().unwrap().to_string();
            let focal = components.next().unwrap().parse().unwrap();
            let holiday_hash = holiday_hash(&label);
            let label = label.into();
            return Put {
                label,
                focal,
                holiday_hash,
            };
        } else if input.contains('-') {
            let mut components = input.split('-');
            let label = components.next().unwrap().to_string();
            let holiday_hash = holiday_hash(&label);
            let label = label.into();
            return Get {
                label,
                holiday_hash,
            };
        }
        panic!("bad command: \"{}\"", input);
    }
}

struct Lens {
    label: Rc<String>,
    focal: usize,
    holiday_hash: usize,
}

impl From<Command> for Lens {
    fn from(cmd: Command) -> Self {
        if let Put {
            label,
            focal,
            holiday_hash,
        } = cmd
        {
            Self {
                label,
                focal,
                holiday_hash,
            }
        } else {
            panic!("can only form Lens from \"Put\" command")
        }
    }
}

fn main() {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for cmd_str in stdin().lock().lines().next().unwrap().unwrap().split(',') {
        let command = cmd_str.into();

        match &command {
            Put { .. } => {
                let new_lens: Lens = command.into();

                if let Some(lenses) = boxes.get_mut(&new_lens.holiday_hash) {
                    if let Some(pos) = lenses
                        .iter()
                        .position(|curr_lens| curr_lens.label == new_lens.label)
                    {
                        *lenses.get_mut(pos).unwrap() = new_lens;
                    } else {
                        lenses.push(new_lens);
                    }
                } else {
                    boxes.insert(new_lens.holiday_hash, vec![new_lens]);
                }
            }
            Get {
                label,
                holiday_hash,
            } => {
                if let Some(lenses) = boxes.get_mut(holiday_hash) {
                    if let Some(pos) = lenses.iter().position(|lens| lens.label == *label) {
                        lenses.remove(pos);
                    }
                }
            }
        }
    }

    let mut total = 0;

    for (boxnum, lenses) in boxes.iter() {
        for (slotnum, lens) in lenses.iter().enumerate() {
            total += ((*boxnum) + 1) * (slotnum + 1) * lens.focal;
        }
    }

    println!("{}", total);
}
