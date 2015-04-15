//#![feature(collections)]
//#![feature(convert)]
extern crate rand;

use rand::{thread_rng, Rng};

use std::io;
use std::option::Option;
use std::str::FromStr;

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
type Hand = Vec<Card>;

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

    print!("{: >#02}{}", value.to_string(), suit.to_string());
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
    thread_rng().shuffle(&mut deck[..]);
}

fn score_hand(hand: &Hand) -> i32 {
    // Number cards are face value, faces are 10, Ace is 11 unless that makes the score >21,
    // then Aces are 1
    let mut score: i32 = 0;
    let mut has_ace: bool = false;
    for card in hand {
        let (ref value, _) = *card;
        match *value {
            Value::Ace => {
                score += 11;
                has_ace = true;
            },
            Value::Jack => score += 10,
            Value::Queen => score += 10,
            Value::King => score += 10,
            _ => score += i32::from_str(value.to_string().as_ref()).unwrap_or(0),
        }
    }

    if has_ace && score > 21 {
        score -=10;
    }

    score
}

fn print_hands(dealer: &Hand, player: &Hand) {
    print!("Dealer:\t");
    for card in dealer {
        print_card(card);
        print!(" ");
    }
    
    print!("\nPlayer:\t");
    for card in player {
        print_card(card);
        print!(" ");
    }
}

fn deal_card(hand: &mut Hand, deck: &mut Deck) {
    hand.push(deck.pop().expect("How do you run out of cards in Blackjack?!"));
}

fn play_hand(deck: &mut Deck) {
    // Each hand begins with a shuffle
    shuffle_deck(deck);

    // Deal to dealer and player, alternately
    let mut dealer_hand = Hand::new();
    let mut player_hand = Hand::new();
    deal_card(&mut dealer_hand, deck);
    deal_card(&mut player_hand, deck);
    deal_card(&mut player_hand, deck);

    // Let player go first
    loop {
        print_hands(&dealer_hand, &player_hand);

        println!("\n[h]it or [s]tand?");

        let mut input = String::new();
        match io::stdin().read_line(&mut input).ok() {
            Option::Some(_) => match input.trim().as_ref() {
                "s" | "S" => break,
                "h" | "H" => {
                    deal_card(&mut player_hand, deck);
                    if score_hand(&player_hand) > 21 {
                        println!("\nBUST!\n");
                        break;
                    }
                },
                _ => println!("Invalid Value!"),
            },
            Option::None => println!("Invalid Value!"),
        }
    }

    //Dealer Logic
    while score_hand(&dealer_hand) < 17 {
        println!("\nDealer hits!");
        deal_card(&mut dealer_hand, deck);
        print_hands(&dealer_hand, &player_hand);
    }
    println!("\nDealer stands!");

    //Final Scoring
    let dealer_score = score_hand(&dealer_hand);
    let player_score = score_hand(&player_hand);
    if dealer_score == 21 && dealer_hand.len() == 2 {
        println!("\nDEALER WINS with blackjack!");
    } else if player_score == 21 && player_hand.len() == 2 {
        println!("\nYOU WIN with blackjack!");
    } else if player_score > 21 {
        println!("\nDEALER WINS, you bust!");
    } else if dealer_score > 21 {
        println!("\nYOU WIN, dealer busts!");
    } else if player_score > dealer_score {
        println!("\nYOU WIN");
    } else {
        println!("\nDEALER WINS");
    }

    //Return cards to deck
    //Note: had to remove drain and use a pop loop instead
    //because drain() is unstable in beta
    loop {
        match dealer_hand.pop() {
            None => break,
            Some(card) => deck.push(card),
        }
    }
    //for card in dealer_hand.drain() {
    //    deck.push(card);
    //}

    loop {
        match dealer_hand.pop() {
            None => break,
            Some(card) => deck.push(card),
        }
    }
    //for card in player_hand.drain() {
    //    deck.push(card);
    //}
}

fn main() {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);

    println!("Welcome to Blackjack!");

    loop {
        println!("\nWould you like to play a hand (y/n)?");

        let mut input = String::new();

        match io::stdin().read_line(&mut input).ok() {
            Option::Some(_) => match input.trim().as_ref() {
                "y" | "yes" => play_hand(&mut deck),
                "n" | "no" => break,
                _ => println!("Please enter 'yes' or 'no'."),
            },
            Option::None => break,
        }
    }
}
