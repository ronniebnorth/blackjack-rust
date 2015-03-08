extern crate blackjack;

use blackjack::deck_of_cards;

fn main() {
    let value = deck_of_cards::Value::Two;
    let suit = deck_of_cards::Suit::Hearts;

    println!("{}{}", value.to_string(), suit.to_string());
}
