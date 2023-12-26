use std::io::{stdin, BufRead};

struct GameCard {
    score: usize,
}

impl From<String> for GameCard {
    fn from(line: String) -> Self {
        let (_, nums_str) = line.split_at(line.find(": ").unwrap());
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

        Self { score }
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

    fn score(num_matched: usize) -> usize {
        if num_matched == 0 {
            return 0;
        }

        let num_matched: u32 = num_matched.try_into().unwrap();
        let num_matched = num_matched.checked_sub(1).unwrap();

        2_usize.pow(num_matched)
    }
}

fn main() {
    let sum: usize = stdin()
        .lock()
        .lines()
        .map(|line| GameCard::from(line.unwrap()))
        .map(|card| card.score)
        .sum();
    println!("{sum}");
}
