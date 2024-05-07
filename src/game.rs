use crate::cards::Deck;
use crate::cards::Card;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::stdin;

struct Game {
    decks: Vec<Deck>,
    all_cards: Vec<Card>,
    choice: String,
    user_cards: Vec<Card>,
    house_cards: Vec<Card>,
    user_sum: u8,
    house_sum: u8
}

impl Game {
    fn create_decks() -> Self {
        let mut decks = Vec::new();
        let mut all_cards = Vec::new();
        let user_cards = Vec::new();
        let house_cards = Vec::new();

        for _ in 0..3 {
            let deck = Deck::create_deck();
            decks.push(deck);
        }

        for deck in &decks {
            for card in &deck.deck {
                all_cards.push(card.clone());
            }
        }

        all_cards.shuffle(&mut thread_rng());

        let choice = "idle".to_string();
        Game { decks, all_cards, user_cards, house_cards, choice, user_sum: 0, house_sum: 0 }
    }

    fn play(&mut self) {
        for _ in 0..2 {
            let random_card = self.get_card();
            self.house_cards.push(random_card.clone());
            self.house_sum += random_card.rank.value();
        }

        while self.choice != "exit" && self.choice != "stand" {
            print!("\x1B[2J\x1B[1;1H");
            let random_card = self.get_card();
            // println!("Card: {:?} of {:?}", random_card.rank, random_card.suit);
            self.user_cards.push(random_card.clone());
            self.user_sum += random_card.rank.value();
            self.display_cards(false);
            if self.user_sum > 21 {
                let message = format!("You lose! Your total sum is: {}", self.user_sum);
                end_program_error(&message);
            }
            self.get_input();
            println!("Choice: {}", self.choice);
            println!("is true: {}", self.choice == "stand");
        }

        // TODO: handle ace values
        if self.choice == "stand" {
            while self.house_sum < 17 {
                let random_card = self.get_card();
                self.house_cards.push(random_card.clone());
                self.house_sum += random_card.rank.value(); 
            }

            print!("\x1B[2J\x1B[1;1H");
            self.display_cards(true);

            if self.house_sum > 21 {
                let message = format!("You win! Your sum is {}. House total sum is: {}", self.user_sum, self.house_sum);
                end_program_error(&message);
            }

            if self.user_sum > self.house_sum {
                let message = format!("You win! Your sum is {}. House total sum is: {}", self.user_sum, self.house_sum);
                end_program_error(&message);
            } else {
                let message = format!("You lose! Your sum is {}. House total sum is: {}", self.user_sum, self.house_sum);
                end_program_error(&message);
            }
        }

        println!("The game has finished");
    }

    fn get_input(&mut self) {
        let mut option = String::new();
        println!("Enter an option: 1. Hit 2. Stand");
        stdin().read_line(&mut option).expect("Failed to read line");
        option = option.trim().to_lowercase();
        self.choice = option;
    }

    fn get_card(&mut self) -> Card {
        let random_index = thread_rng().gen_range(0..self.all_cards.len());

        let random_card = &self.all_cards[random_index].clone();

        self.all_cards.remove(random_index);

        // println!("Random card: {:?} of {:?}", random_card.rank, random_card.suit);
        random_card.clone()
    }

    /* Displays the house's and the user's cards */
    fn display_cards(&self, show_cards: bool) {
        println!("House cards:");
        for (i, card) in self.house_cards.iter().enumerate() {
            if i==0 && !show_cards {
                println!("<Face down card>");
                continue;
            }

            println!("{:?} of {:?}", card.rank, card.suit);
        }

        println!("User cards:");
        for card in &self.user_cards {
            println!("{:?} of {:?}", card.rank, card.suit);
        }
    }
}

pub fn setup() {
    let mut game = Game::create_decks();
    let mut start_option = String::new();
    println!("Welcome to blackjack! Do you want to start? (y/n)");
    stdin().read_line(&mut start_option).expect("Failed to read line");

    game.play();
}

// TODO: change name
fn end_program_error(message: &str) {
    println!("{message}");
    std::process::exit(0);
}