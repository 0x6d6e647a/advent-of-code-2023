use std::io::{stdin, BufRead, Stdin};

fn parse(stdin: &Stdin) -> Vec<Vec<i64>> {
    stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_next(nums: &[i64]) -> i64 {
    let mut sum = 0;
    let mut nums = nums.to_owned();

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

fn main() {
    let num_lines = parse(&stdin());
    let sum: i64 = num_lines.iter().map(|nums| find_next(nums)).sum();
    println!("{sum}");
}
