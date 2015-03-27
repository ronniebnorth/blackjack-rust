#![feature(convert)]
extern crate rand;

use rand::{thread_rng, Rng};

use std::io;
use std::option::Option;

#[allow(dead_code)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[allow(dead_code)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

type Card = (Value, Suit);

type Deck = Vec<Card>;

impl ToString for Value {
    fn to_string(&self) -> String {
        match *self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        }.to_string()
    }
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match *self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }.to_string()
    }
}

fn print_card(card: &Card) {
    let (ref value, ref suit) = *card;

    println!("{: >#02}{}", value.to_string(), suit.to_string());
}

fn create_deck() -> Deck {
    vec![
        (Value::Ace, Suit::Clubs),
        (Value::Two, Suit::Clubs),
        (Value::Three, Suit::Clubs),
        (Value::Four, Suit::Clubs),
        (Value::Five, Suit::Clubs),
        (Value::Six, Suit::Clubs),
        (Value::Seven, Suit::Clubs),
        (Value::Eight, Suit::Clubs),
        (Value::Nine, Suit::Clubs),
        (Value::Ten, Suit::Clubs),
        (Value::Jack, Suit::Clubs),
        (Value::Queen, Suit::Clubs),
        (Value::King, Suit::Clubs),
        (Value::Ace, Suit::Clubs),

        (Value::Two, Suit::Diamonds),
        (Value::Three, Suit::Diamonds),
        (Value::Four, Suit::Diamonds),
        (Value::Five, Suit::Diamonds),
        (Value::Six, Suit::Diamonds),
        (Value::Seven, Suit::Diamonds),
        (Value::Eight, Suit::Diamonds),
        (Value::Nine, Suit::Diamonds),
        (Value::Ten, Suit::Diamonds),
        (Value::Jack, Suit::Diamonds),
        (Value::Queen, Suit::Diamonds),
        (Value::King, Suit::Diamonds),

        (Value::Ace, Suit::Hearts),
        (Value::Two, Suit::Hearts),
        (Value::Three, Suit::Hearts),
        (Value::Four, Suit::Hearts),
        (Value::Five, Suit::Hearts),
        (Value::Six, Suit::Hearts),
        (Value::Seven, Suit::Hearts),
        (Value::Eight, Suit::Hearts),
        (Value::Nine, Suit::Hearts),
        (Value::Ten, Suit::Hearts),
        (Value::Jack, Suit::Hearts),
        (Value::Queen, Suit::Hearts),
        (Value::King, Suit::Hearts),

        (Value::Ace, Suit::Spades),
        (Value::Two, Suit::Spades),
        (Value::Three, Suit::Spades),
        (Value::Four, Suit::Spades),
        (Value::Five, Suit::Spades),
        (Value::Six, Suit::Spades),
        (Value::Seven, Suit::Spades),
        (Value::Eight, Suit::Spades),
        (Value::Nine, Suit::Spades),
        (Value::Ten, Suit::Spades),
        (Value::Jack, Suit::Spades),
        (Value::Queen, Suit::Spades),
        (Value::King, Suit::Spades),
    ]
}

fn shuffle_deck(deck: &mut Deck) {
    thread_rng().shuffle(deck.as_mut_slice());
}

fn play_hand(deck: &mut Deck) {
    println!("You play a hand!");
}

fn main() {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);

    println!("Welcome to Blackjack!");

    loop {
        println!("Would you like to play a hand (y/n)?");

        let mut input = String::new();

        match io::stdin().read_line(&mut input).ok() {
            Option::Some(_) => match input.trim().as_ref() {
                "y" => play_hand(&mut deck),
                "yes" => play_hand(&mut deck),
                "n" => break,
                "no" => break,
                _ => println!("Please enter 'yes' or 'no'."),
            },
            Option::None => break,
        }
    }
}
