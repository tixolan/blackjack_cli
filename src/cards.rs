use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub enum Suits {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suits {
    fn value(&self) -> &str {
        match *self {
            Suits::Hearts => "♡",
            Suits::Spades => "♠",
            Suits::Diamonds => "♢",
            Suits::Clubs => "♣",
        }
    }

    fn iterator() -> impl Iterator<Item = Suits> {
        let iterator = [
            Suits::Hearts,
            Suits::Spades,
            Suits::Diamonds,
            Suits::Clubs,
        ].iter().copied();

        iterator
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Symbols {
    ACE,
    KING,
    QUEEN,
    JACK,
    TEN,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
}

impl Symbols {
    fn value(&self) -> &str {
        match *self {
            Symbols::ACE => "A",
            Symbols::KING => "K",
            Symbols::QUEEN => "Q",
            Symbols::JACK => "J",
            Symbols::TEN => "10",
            Symbols::NINE => "9",
            Symbols::EIGHT => "8",
            Symbols::SEVEN => "7",
            Symbols::SIX => "6",
            Symbols::FIVE => "5",
            Symbols::FOUR => "4",
            Symbols::THREE => "3",
            Symbols::TWO => "2",
        }
    }

    pub fn score(&self) -> i32 {
        match *self {
            Symbols::ACE => 11,
            Symbols::KING => 10,
            Symbols::QUEEN => 10,
            Symbols::JACK => 10,
            Symbols::TEN => 10,
            Symbols::NINE => 9,
            Symbols::EIGHT => 8,
            Symbols::SEVEN => 7,
            Symbols::SIX => 6,
            Symbols::FIVE => 5,
            Symbols::FOUR => 4,
            Symbols::THREE => 3,
            Symbols::TWO => 2,
        }
    }

    fn iterator() -> impl Iterator<Item = Symbols> {
        let iterator = [
            Symbols::ACE,
            Symbols::KING,
            Symbols::QUEEN,
            Symbols::JACK,
            Symbols::TEN,
            Symbols::NINE,
            Symbols::EIGHT,
            Symbols::SEVEN,
            Symbols::SIX,
            Symbols::FIVE,
            Symbols::FOUR,
            Symbols::THREE,
            Symbols::TWO,
        ].iter().copied();

        iterator
    }
}

pub struct Card {
    pub _suit: Suits,
    pub _symbol: Symbols,
    pub lines: Vec<String>,
}

impl Card {
    pub fn _display(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }
}

static TOP_LINE: &str    = "┌─────────┐";
static BOTTOM_LINE: &str = "└─────────┘";
static BLANK_LINE: &str  = "│         │";

fn gen_suit_line(suit: Suits, double: bool) -> String {
    if double {
        format!("│   {} {}   │", suit.value(), suit.value())
    } else {
        format!("│    {}    │", suit.value())
    }
}

pub fn gen_card_lines(symbol: Symbols, suit: Suits) -> Vec<String> {
    let symbol_line_top = format!("│ {}{}│", symbol.value(), " ".repeat(8 - symbol.value().len()));
    let symbol_line_bottom = format!("│{}{} │", " ".repeat(8 - symbol.value().len()), symbol.value());

    let mut card = vec![
        TOP_LINE.to_owned(),
        symbol_line_top,

    ];

    match symbol {
        Symbols::ACE | Symbols::KING | Symbols::QUEEN | Symbols::JACK => {
            card.push(BLANK_LINE.to_owned());
            card.push(BLANK_LINE.to_owned());
            card.push(gen_suit_line(suit, false));
            card.push(BLANK_LINE.to_owned());
            card.push(BLANK_LINE.to_owned());
        },
        Symbols::TWO | Symbols::FOUR => {
            card.push(BLANK_LINE.to_owned());
            card.push(gen_suit_line(suit, symbol == Symbols::FOUR));
            card.push(BLANK_LINE.to_owned());
            card.push(gen_suit_line(suit, symbol == Symbols::FOUR));
            card.push(BLANK_LINE.to_owned());
        },
        Symbols::THREE => {
            card.push(BLANK_LINE.to_owned());
            card.push(gen_suit_line(suit, false));
            card.push(gen_suit_line(suit, false));
            card.push(gen_suit_line(suit, false));
            card.push(BLANK_LINE.to_owned());
        },
        Symbols::FIVE | Symbols::SIX => {
            card.push(BLANK_LINE.to_owned());
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, symbol == Symbols::SIX));
            card.push(gen_suit_line(suit, true));
            card.push(BLANK_LINE.to_owned());
        },
        Symbols::SEVEN | Symbols::EIGHT => {
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, symbol == Symbols::EIGHT));
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, true));
            card.push(BLANK_LINE.to_owned());
        },
        Symbols::NINE | Symbols::TEN => {
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, symbol == Symbols::TEN));
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, true));
            card.push(gen_suit_line(suit, true));
        }
    }


    card.append(&mut vec![
        symbol_line_bottom,
        BOTTOM_LINE.to_owned(),
    ]);

    card
}

pub fn get_blank_card_lines() -> Vec<String> {
    vec![
        String::from("┌─────────┐"),
        String::from("│   ___   │"),
        String::from("│  /\\  \\  │"),
        String::from("│  \\:\\  \\ │"),
        String::from("│  /::\\__\\│"),
        String::from("│ /:/\\/__/│"),
        String::from("│ \\/__/   │"),
        String::from("│         │"),
        String::from("└─────────┘"),
    ]
}

pub fn gen_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    for suit in Suits::iterator() {
        for symbol in Symbols::iterator() {
            let lines = gen_card_lines(symbol, suit);
            cards.push(Card{
                _suit: suit,
                _symbol: symbol,
                lines,
            });
        }
    }

    cards.shuffle(&mut thread_rng());

    cards
}

pub fn display_cards(cards: &Vec<Card>, end_with_blank: bool) {
    for i in 0..9 {
        for card in cards {
            print!("{} ", card.lines[i]);
            if end_with_blank {
                print!("{} ", get_blank_card_lines()[i]);
            }
        }
        println!()
    }
}