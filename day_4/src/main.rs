use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
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

    fn score_length(&self) -> usize {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .collect::<HashSet<_>>()
            .len()
    }

    fn score(&self) -> u32 {
        let length = self.score_length();

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

struct CardWins {
    cards: Vec<Card>,
    won_duplicates: HashMap<u32, u32>,
}

impl CardWins {
    fn build(cards: Vec<Card>) -> Self {
        CardWins {
            cards,
            won_duplicates: HashMap::new(),
        }
    }

    fn get_won_duplicates(&self, card_id: u32) -> u32 {
        match self.won_duplicates.get(&card_id) {
            Some(&num) => num,
            None => 0,
        }
    }

    fn get_won_cards(&mut self, card: &Card) -> u32 {
        let cards_length = self.cards.len();
        let count = card.score_length();
        let curr_card_duplicates = self.get_won_duplicates(card.number);

        let upper_bound = if card.number + count as u32 > cards_length as u32 {
            (cards_length - 1) as u32
        } else {
            card.number + count as u32
        };

        for card_id in (card.number + 1)..=upper_bound {
            let duplicates = self.get_won_duplicates(card_id);
            self.won_duplicates
                .insert(card_id, duplicates + curr_card_duplicates + 1);
        }

        return 1 + curr_card_duplicates;
    }

    fn get_score(&self) -> u32 {
        self.cards.iter().fold(0, |acc, c| acc + c.score())
    }

    fn get_total_cards(&mut self) -> u32 {
        // There must be a better way to handle borrowing here
        let cards = self.cards.clone();

        cards.iter().fold(0, |acc, c| acc + self.get_won_cards(c))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input file");

    let cards: Vec<Card> = input
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    let mut card_wins = CardWins::build(cards);

    // Part 1
    println!("{}", card_wins.get_score());

    // Part 2
    println!("{}", card_wins.get_total_cards());
}
