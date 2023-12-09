use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_next(nums: &[i64], reverse: bool) -> i64 {
    let mut sum = 0;
    let mut nums = nums.to_owned();

    if reverse {
        nums.reverse();
    }

    while !nums.iter().all(|n| *n == 0) {
        let mut new_nums = Vec::new();

        let mut nums_iter = nums.into_iter();
        let mut curr = nums_iter.next().unwrap();

        for next in nums_iter {
            new_nums.push(next - curr);
            curr = next;
        }

        sum += curr;
        nums = new_nums;
    }

    sum
}

#[aoc(day9, part1)]
fn part1(num_lines: &[Vec<i64>]) -> i64 {
    num_lines.iter().map(|nums| find_next(nums, false)).sum()
}

#[aoc(day9, part2)]
fn part2(num_lines: &[Vec<i64>]) -> i64 {
    num_lines.iter().map(|nums| find_next(nums, true)).sum()
}
