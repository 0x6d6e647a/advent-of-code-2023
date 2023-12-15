use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn holiday_hash(input: &str) -> usize {
    let mut value = 0;

    for char in input.chars() {
        value += char as usize;
        value *= 17;
        value %= 256;
    }

    value
}

enum Command<'a> {
    Put {
        label: &'a str,
        focal: usize,
        holiday_hash: usize,
    },
    Get {
        label: &'a str,
        holiday_hash: usize,
    },
}

use Command::*;

impl Command<'_> {
    fn new(input: &str) -> Command {
        if input.contains('=') {
            let mut components = input.split('=');
            let label = components.next().unwrap();
            let focal = components.next().unwrap().parse().unwrap();
            let holiday_hash = holiday_hash(label);
            return Put {
                label,
                focal,
                holiday_hash,
            };
        } else if input.contains('-') {
            let mut components = input.split('-');
            let label = components.next().unwrap();
            let holiday_hash = holiday_hash(label);
            return Get {
                label,
                holiday_hash,
            };
        }
        panic!("bad command: \"{}\"", input);
    }
}

struct Lens<'a> {
    label: &'a str,
    focal: usize,
    holiday_hash: usize,
}

impl<'a> TryFrom<Command<'a>> for Lens<'a> {
    type Error = &'static str;

    fn try_from(cmd: Command<'a>) -> Result<Self, Self::Error> {
        if let Put {
            label,
            focal,
            holiday_hash,
        } = cmd
        {
            Ok(Self {
                label,
                focal,
                holiday_hash,
            })
        } else {
            Err("cannot form Lens from \"Get\" command")
        }
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input.split(',').map(holiday_hash).sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for cmd_str in input.split(',') {
        let command = Command::new(cmd_str);

        match &command {
            Put { .. } => {
                let new_lens: Lens = command.try_into().unwrap();

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

    total
}
