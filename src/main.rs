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

//type Deck = Vec<Card>;

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

fn print_card(card: Card) {
    let (value, suit) = card;

    println!("{: >#02}{}", value.to_string(), suit.to_string());
}

fn main() {
    let card: Card = (Value::Ten, Suit::Clubs);

    //println!("{}{}", value.to_string(), suit.to_string());
    print_card(card);
}
