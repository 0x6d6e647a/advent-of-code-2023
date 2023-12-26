use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
};

struct GameCard {
    index: usize,
    num_matched: usize,
}

impl From<String> for GameCard {
    fn from(line: String) -> Self {
        let (card_str, nums_str) = line.split_at(line.find(": ").unwrap());
        let index = card_str.split_whitespace().nth(1).unwrap().parse().unwrap();
        let (winning_str, your_str) = nums_str.split_at(nums_str.find(" | ").unwrap());

        let winning_nums: Vec<u8> = winning_str
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let your_nums: Vec<u8> = your_str
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let num_matched = Self::num_matched(winning_nums, your_nums);

        Self {
            index,
            num_matched,
        }
    }
}

impl GameCard {
    fn num_matched(winning_nums: Vec<u8>, your_nums: Vec<u8>) -> usize {
        let mut num_matched = 0;

        for winning_num in &winning_nums {
            for your_num in &your_nums {
                if winning_num == your_num {
                    num_matched += 1;
                    break;
                }
            }
        }

        num_matched
    }
}

fn main() {
    let cards: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| GameCard::from(line.unwrap()))
        .collect();
    let mut queue: VecDeque<_> = cards.iter().map(|card| card.index).collect();

    let mut total = 0;

    while let Some(curr_index) = queue.pop_front() {
        total += 1;

        let num_matched = cards.get(curr_index - 1).unwrap().num_matched;

        for new_index in curr_index + 1..=curr_index + num_matched {
            queue.push_back(new_index);
        }
    }

    println!("{total}");
}
