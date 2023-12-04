use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct GameCard {
    index: usize,
    num_matched: usize,
    score: u32,
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

    fn score(num_matched: usize) -> u32 {
        if num_matched == 0 {
            return 0;
        }

        2_u32.pow(num_matched as u32 - 1)
    }

    fn new(line: &str) -> Self {
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
        let score = Self::score(num_matched);

        GameCard {
            index,
            num_matched,
            score,
        }
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<GameCard> {
    input.lines().map(GameCard::new).collect()
}

#[aoc(day4, part1)]
fn part1(cards: &[GameCard]) -> u32 {
    cards.iter().map(|c| c.score).sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[GameCard]) -> u32 {
    let mut total = 0;
    let mut curr_cards: VecDeque<usize> = cards.iter().map(|c| c.index).collect();

    while !curr_cards.is_empty() {
        total += 1;

        let curr_index = curr_cards.pop_front().unwrap();
        let num_matched = cards.get(curr_index - 1_usize).unwrap().num_matched;

        for new_index in curr_index + 1..=curr_index + num_matched {
            curr_cards.push_back(new_index);
        }
    }

    total
}
