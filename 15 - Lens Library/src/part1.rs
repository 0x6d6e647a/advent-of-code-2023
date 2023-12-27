use std::io::{stdin, BufRead};

fn holiday_hash(input: &str) -> usize {
    let mut value = 0;

    for char in input.chars() {
        value += char as usize;
        value *= 17;
        value %= 256;
    }

    value
}

fn main() {
    let sum: usize = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(holiday_hash)
        .sum();
    println!("{}", sum);
}
