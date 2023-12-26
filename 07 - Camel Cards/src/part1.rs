use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum CardType {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

use CardType::*;

impl From<char> for CardType {
    fn from(ch: char) -> Self {
        match ch {
            '2' => C2,
            '3' => C3,
            '4' => C4,
            '5' => C5,
            '6' => C6,
            '7' => C7,
            '8' => C8,
            '9' => C9,
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            _ => panic!("not a card face: '{}'", ch),
        }
    }
}

impl CardType {
    fn value(&self) -> usize {
        match self {
            C2 => 0,
            C3 => 1,
            C4 => 2,
            C5 => 3,
            C6 => 4,
            C7 => 5,
            C8 => 6,
            C9 => 7,
            T => 8,
            J => 9,
            Q => 10,
            K => 11,
            A => 12,
        }
    }
}

const HAND_SIZE: usize = 5;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use HandType::*;

impl HandType {
    fn new(card_count_map: &HashMap<CardType, usize>) -> Self {
        let mut card_type_counts: Vec<_> = card_count_map.iter().map(|(_, n)| n).collect();
        card_type_counts.sort_by(|a, b| b.cmp(a));
        let mut card_type_counts = card_type_counts.into_iter();

        let a = match card_type_counts.next() {
            Some(a) => a,
            _ => return HighCard,
        };

        match a {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 | 2 => {
                let b = card_type_counts.next().unwrap_or(&0_usize);
                match (a, b) {
                    (3, 2) => FullHouse,
                    (3, _) => ThreeOfAKind,
                    (2, 2) => TwoPair,
                    (2, _) => OnePair,
                    _ => panic!("unhandled card count case: ({}, {})", a, b),
                }
            }
            1 => HighCard,
            _ => panic!("unhandled card count case: {}", a),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [CardType; HAND_SIZE],
    bid: usize,
    hand_type: HandType,
}

impl From<String> for Hand {
    fn from(line: String) -> Self {
        let mut components = line.split_whitespace();

        // -- Get cards.
        let mut cards = [CardType::A; HAND_SIZE];
        components
            .next()
            .unwrap()
            .chars()
            .map(|ch| ch.into())
            .zip(cards.iter_mut())
            .for_each(|(card_type, slot)| *slot = card_type);

        // -- Get bid
        let bid = components.next().unwrap().parse().unwrap();

        // -- Count cards.
        let mut card_count_map = HashMap::new();

        for card_type in cards.iter() {
            if let Some(n) = card_count_map.get_mut(card_type) {
                *n += 1;
            } else {
                card_count_map.insert(*card_type, 1);
            }
        }

        // -- Determine hand type.
        let hand_type = HandType::new(&card_count_map);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.hand_type.cmp(&other.hand_type);

        if ord != Ordering::Equal {
            return ord;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(c0, c1)| c0.value().cmp(&c1.value()))
            .find(|ord| *ord != Ordering::Equal)
            .unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut hands: Vec<Hand> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().into())
        .collect();
    hands.sort();
    let solution: usize = hands.iter().enumerate().map(|(i, h)| h.bid * (i + 1)).sum();
    println!("{solution}");
}
