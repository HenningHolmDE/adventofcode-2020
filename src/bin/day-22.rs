use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader};

type Card = u32;

#[derive(Clone, Debug)]
struct Deck {
    cards: Vec<u32>,
}

impl Deck {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn draw(&mut self) -> Card {
        self.cards.remove(0)
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn score(&self) -> u32 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i as u32 + 1) * c)
            .sum()
    }

    fn copy_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    fn copy_subdeck(&self, n: usize) -> Self {
        Self {
            cards: self.cards[0..n].into(),
        }
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug)]
struct GameResult {
    winning_player: usize,
    score: u32,
    max_game_id: u32,
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    decks: Vec<Deck>,
    debug: bool,
    recursion: bool,
    state_history: Vec<u64>,
}

impl Game {
    fn new(decks: &Vec<Deck>) -> Self {
        Self::new_inner(1, decks)
    }

    fn new_inner(id: u32, decks: &Vec<Deck>) -> Self {
        assert_eq!(decks.len(), 2, "Number of players should be 2");
        Self {
            id,
            decks: decks.clone(),
            debug: false,
            recursion: false,
            state_history: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn enable_debug(&mut self) {
        self.debug = true;
    }

    fn enable_recursion(&mut self) {
        self.recursion = true;
    }

    fn play(&mut self) -> GameResult {
        if self.debug {
            println!("=== Game {} ===", self.id);
        }
        let mut round = 0;
        let mut max_game_id = self.id;
        let mut winning_player = 0;
        while self.all_players_have_cards() {
            round += 1;
            if self.debug {
                println!("");
                println!("-- Round {} (Game {}) --", round, self.id);
                self.decks
                    .iter()
                    .enumerate()
                    .for_each(|(i, p)| println!("Player {}'s deck: {}", i + 1, p));
            }
            // check if configuration is known
            let mut hasher = DefaultHasher::new();
            self.decks
                .iter()
                .map(|d| d.copy_cards())
                .collect::<Vec<_>>()
                .hash(&mut hasher);
            let state = hasher.finish();
            if self.state_history.contains(&state) {
                if self.debug {
                    println!("Known state detected!");
                }
                winning_player = 0;
                break;
            }
            self.state_history.push(state);
            // draw cards
            let cards: Vec<_> = self.decks.iter_mut().map(|p| p.draw()).collect();
            if self.debug {
                cards
                    .iter()
                    .enumerate()
                    .for_each(|(i, c)| println!("Player {} plays: {}", i + 1, c));
            }
            // check recursion
            let recurse = self.recursion
                && cards
                    .iter()
                    .enumerate()
                    .all(|(i, &c)| self.decks[i].len() >= c as usize);
            if recurse {
                // winner of inner game wins this round
                if self.debug {
                    println!("Playing a sub-game to determine the winner...");
                    println!("");
                }
                let decks = cards
                    .iter()
                    .enumerate()
                    .map(|(i, &c)| self.decks[i].copy_subdeck(c as usize))
                    .collect();
                let mut game = Self::new_inner(max_game_id + 1, &decks);
                game.enable_recursion();
                if self.debug {
                    game.enable_debug();
                }
                let result = game.play();
                if self.debug {
                    println!("...anyway, back to game {}.", self.id);
                }
                max_game_id = result.max_game_id;
                winning_player = result.winning_player;
            } else {
                // highest cards wins
                let winning_card = cards.iter().max().unwrap();
                winning_player = cards.iter().position(|c| c == winning_card).unwrap();
            };
            if self.debug {
                println!(
                    "Player {} wins round {} of game {}!",
                    winning_player + 1,
                    round,
                    self.id
                );
            }
            // winning player gets the cards (winning card first)
            self.decks[winning_player].add(cards[winning_player]);
            self.decks[winning_player].add(cards[1 - winning_player]);
        }
        if self.debug {
            println!(
                "The winner of game {} is player {}!",
                self.id,
                winning_player + 1
            );
            println!("");
            if self.id == 1 {
                println!("");
                println!("== Post-game results ==");
                self.decks
                    .iter()
                    .enumerate()
                    .for_each(|(i, p)| println!("Player {}'s deck: {}", i + 1, p));
                println!("");
            }
        }
        GameResult {
            winning_player,
            score: self.decks[winning_player].score(),
            max_game_id,
        }
    }

    fn all_players_have_cards(&self) -> bool {
        self.decks.iter().all(|p| p.len() > 0)
    }
}

#[derive(Debug)]
enum LoadState {
    Idle,
    Deck,
}

fn load_decks_from_file(filename: &str) -> Result<Vec<Deck>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut load_state = LoadState::Idle;
    let mut decks = Vec::new();
    let mut deck = Deck::new();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                if deck.len() > 0 {
                    let deck = std::mem::replace(&mut deck, Deck::new());
                    decks.push(deck);
                }
                load_state = LoadState::Idle;
            } else {
                let line = line.trim();
                match load_state {
                    LoadState::Idle => {
                        if line.starts_with("Player") {
                            load_state = LoadState::Deck;
                        }
                    }
                    LoadState::Deck => deck.add(line.parse().unwrap()),
                }
            }
        }
    }
    if deck.len() > 0 {
        decks.push(deck);
    }
    Ok(decks)
}

fn part1(decks: &Vec<Deck>) {
    let mut game = Game::new(decks);
    // game.enable_debug();
    let result = game.play();
    println!(
        "Part 1: Player {} wins the game with this score: {}",
        result.winning_player + 1,
        result.score
    );
}

fn part2(decks: &Vec<Deck>) {
    let mut game = Game::new(decks);
    game.enable_recursion();
    // game.enable_debug();
    let result = game.play();
    println!(
        "Part 2: Player {} wins the game with this score: {}",
        result.winning_player + 1,
        result.score
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let decks = load_decks_from_file("inputs/day-22.txt")?;
    part1(&decks);
    part2(&decks);
    Ok(())
}
