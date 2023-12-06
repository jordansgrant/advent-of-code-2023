use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[allow(dead_code)]
struct Card {
    number: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    fn build(number: u32) -> Card {
        Card {
            number,
            winning_numbers: HashSet::new(),
            card_numbers: HashSet::new(),
        }
    }

    fn add_winning_number(&mut self, number: u32) {
        self.winning_numbers.insert(number);
    }

    fn add_card_number(&mut self, number: u32) {
        self.card_numbers.insert(number);
    }

    fn score(&self) -> u32 {
        let intersection: HashSet<_> = self
            .winning_numbers
            .intersection(&self.card_numbers)
            .collect();
        let length = intersection.len();

        if length == 0 {
            return 0;
        }

        u32::pow(2, (length - 1) as u32)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id_str, rest) = s
            .strip_prefix("Card")
            .and_then(|s| s.split_once(":"))
            .unwrap();
        let filtered_card_str: String = card_id_str.chars().filter(|&s| s.is_digit(10)).collect();
        let card_id = filtered_card_str
            .parse::<u32>()
            .map_err(|_| ParseCardError)?;
        let mut card = Card::build(card_id);

        let (winning_numbers_str, card_numbers_str) = rest.split_once("|").unwrap();

        for s in winning_numbers_str.split(" ").filter(|&s| !s.is_empty()) {
            let num = s.parse::<u32>().unwrap();
            card.add_winning_number(num);
        }

        for s in card_numbers_str.split(" ").filter(|&s| !s.is_empty()) {
            let num = s.parse::<u32>().unwrap();
            card.add_card_number(num);
        }

        return Ok(card);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");

    let cards: Vec<Card> = input
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    let total_score = cards
        .iter()
        .map(|c| c.score())
        .fold(0, |acc, score| acc + score);

    println!("{}", total_score)
}
