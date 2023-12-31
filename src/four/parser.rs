use regex::Regex;
use std::collections::HashMap;
use std::str;

pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
    matches: Vec<u32>,
}

impl Card {
    pub fn new(id: u32) -> Card {
        Card {
            id,
            winning_numbers: Vec::new(),
            owned_numbers: Vec::new(),
            matches: Vec::new(),
        }
    }
}

pub struct Parser {
    cards: Vec<Card>,
    re: Regex,
    last_id: u32,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            cards: Vec::new(),
            re: Regex::new(r"Card\s*?(?P<id>\d+):\s(?P<winning>.*?)\s\|\s(?P<owned>.*?)$").unwrap(),
            last_id: 0,
        }
    }

    pub fn read(&mut self, line: &str) {
        let mut id: u32 = 0;

        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut owned_numbers: Vec<u32> = Vec::new();
        let mut matches: Vec<u32> = Vec::new();

        self.last_id += 1;

        for capture in self.re.captures_iter(line) {
            if let Some(card_id) = capture
                .name("id")
                .and_then(|id| id.as_str().parse::<u32>().ok())
            {
                id = card_id;
            }

            if let Some(winning) = capture.name("winning") {
                winning_numbers = winning
                    .as_str()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect();
            }

            if let Some(owned) = capture.name("owned") {
                owned_numbers = owned
                    .as_str()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect();
            }
        }

        if id == 0 {
            return;
        }

        for winning_number in &winning_numbers {
            if owned_numbers.contains(winning_number) {
                matches.push(*winning_number);
            }
        }

        if matches.len() <= 0 {
            return;
        }

        self.cards.push(Card {
            id,
            winning_numbers,
            owned_numbers,
            matches,
        })
    }

    pub fn solution_1(&self) -> u32 {
        let mut solution: u32 = 0;

        for card in self.cards.iter() {
            solution += u32::pow(2, card.matches.len() as u32 - 1);
        }

        solution
    }

    pub fn solution_2(&self) -> u32 {
        let mut solution: u32 = 0;
        let mut copies: HashMap<u32, u32> = HashMap::new();

        for card in self.cards.iter() {
            let copies_of_match = *copies.get(&card.id).unwrap_or(&0);
            let mut next_id = card.id + 1;

            while next_id <= card.id + card.matches.len() as u32 && next_id <= self.last_id {
                let entry = copies.entry(next_id).or_insert(0);

                *entry += copies_of_match;
                *entry += 1;
                next_id += 1;
            }
        }

        solution += self.last_id + copies.values().sum::<u32>();

        solution
    }
}
