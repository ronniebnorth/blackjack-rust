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

//type Card = (Value, Suit);

//type Deck = Vec<Card>;

impl Value {
    pub fn to_string(&self) -> &str {
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
        }
    }
}

impl Suit {
    pub fn to_string(&self) -> &str {
        match *self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }
    }
}

