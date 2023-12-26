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
        use CardType::*;
        match self {
            J => 0,
            C2 => 1,
            C3 => 2,
            C4 => 3,
            C5 => 4,
            C6 => 5,
            C7 => 6,
            C8 => 7,
            C9 => 8,
            T => 9,
            Q => 10,
            K => 11,
            A => 12,
        }
    }
}

const HAND_SIZE: usize = 5;
const WILDCARD: CardType = CardType::J;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    fn new_wildcard(card_count_map: &HashMap<CardType, usize>, num_wildcards: usize) -> Self {
        use HandType::*;

        // -- Eary exit when no wildcards.
        if num_wildcards == 0 {
            return HandType::new(card_count_map);
        }

        // -- Calcuate wildcard upgrade.
        let card_count_map_no_wildcards: HashMap<_, _> = card_count_map
            .iter()
            .filter(|(ct, _)| **ct != WILDCARD)
            .map(|(ct, n)| (*ct, *n))
            .collect();
        let hand_type = HandType::new(&card_count_map_no_wildcards);

        match num_wildcards {
            1 => match hand_type {
                HighCard => OnePair,
                OnePair => ThreeOfAKind,
                TwoPair => FullHouse,
                ThreeOfAKind => FourOfAKind,
                FourOfAKind => FiveOfAKind,
                _ => panic!("unhandled 1 wildcard scenario: {:?}", hand_type),
            },
            2 => match hand_type {
                HighCard => ThreeOfAKind,
                OnePair => FourOfAKind,
                ThreeOfAKind => FiveOfAKind,
                _ => panic!("unhandled 2 wildcard scenario: {:?}", hand_type),
            },
            3 => match hand_type {
                HighCard => FourOfAKind,
                OnePair => FiveOfAKind,
                _ => panic!("unhandled 3 wildcard scenario: {:?}", hand_type),
            },
            4 => FiveOfAKind,
            5 => FiveOfAKind,
            _ => panic!("invalid number of wildcard: {}", num_wildcards),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [CardType; HAND_SIZE],
    bid: usize,
    wildcard_hand_type: HandType,
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

        // -- Find if contains joker.
        let num_wildcards = cards.iter().filter(|c| **c == WILDCARD).count();
        let wildcard_hand_type = HandType::new_wildcard(&card_count_map, num_wildcards);

        Hand {
            cards,
            bid,
            wildcard_hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.wildcard_hand_type.cmp(&other.wildcard_hand_type);

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
