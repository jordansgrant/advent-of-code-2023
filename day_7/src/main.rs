use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: &char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            _ => Card::A,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn build(hand_counts: &HashMap<Card, usize>) -> Self {
        if hand_counts.len() == 5 {
            return Self::HighCard;
        } else if hand_counts.len() == 4 {
            return Self::OnePair;
        } else if hand_counts.len() == 1 {
            return Self::FiveOfAKind;
        } else if hand_counts.len() == 3 {
            if hand_counts.values().any(|v| *v == 3) {
                return Self::ThreeOfAKind;
            }
            return Self::TwoPair;
        } else if hand_counts.values().any(|v| *v == 4) {
            return Self::FourOfAKind;
        }

        Self::FullHouse
    }

    fn build_part_2(hand: &Hand) -> Self {
        let joker_count: usize = match hand.hand_counts.get(&Card::J) {
            Some(count) => *count,
            None => 0,
        };

        if joker_count == 0 || hand.hand_type == Self::FiveOfAKind {
            return hand.hand_type;
        }

        if hand.hand_type == Self::FourOfAKind {
            return Self::FiveOfAKind;
        } else if hand.hand_type == Self::FullHouse {
            return Self::FiveOfAKind;
        } else if hand.hand_type == Self::ThreeOfAKind {
            return Self::FourOfAKind;
        } else if hand.hand_type == Self::HighCard {
            return Self::OnePair;
        } else if hand.hand_type == Self::OnePair {
            return Self::ThreeOfAKind;
        }

        // Hand type is TwoPair
        if joker_count == 1 {
            return Self::FullHouse;
        }
        return Self::FourOfAKind;
    }
}

#[derive(Debug, Eq)]
struct Hand {
    hand: Vec<Card>,
    hand_type: HandType,
    hand_counts: HashMap<Card, usize>,
    score: u32,
}

impl Hand {
    fn build(hand_str: &str, score: u32) -> Self {
        let hand_counts =
            hand_str
                .chars()
                .map(|c| Card::from_char(&c))
                .fold(HashMap::new(), |mut acc, card| {
                    match acc.get(&card) {
                        Some(v) => acc.insert(card, v + 1),
                        None => acc.insert(card, 1),
                    };
                    acc
                });

        Hand {
            hand: hand_str.chars().map(|c| Card::from_char(&c)).collect(),
            hand_type: HandType::build(&hand_counts),
            hand_counts,
            score,
        }
    }

    fn recompute_hand_type_part2(&mut self) {
        self.hand_type = HandType::build_part_2(&self);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.hand.cmp(&other.hand),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.hand == other.hand
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");
    let mut hands: Vec<Hand> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (hand_str, score_str) = l.split_once(" ").unwrap();
            let score = score_str.parse::<u32>().unwrap();

            Hand::build(hand_str, score)
        })
        .collect();

    // Part 1
    hands.sort_unstable();

    let full_score = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.score * (i + 1) as u32);
    println!("{}", full_score);

    // Part 2
    for hand in hands.iter_mut() {
        hand.recompute_hand_type_part2();
    }

    hands.sort_unstable();

    let full_score = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.score * (i + 1) as u32);
    println!("{}", full_score);
}
