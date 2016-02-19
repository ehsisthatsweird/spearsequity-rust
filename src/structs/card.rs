use structs::Rank;
use structs::Suit;


lazy_static! {
    pub static ref CARD_VALUES: [Card; 52] = {
        let mut result: [Card; 52] = [Card::default(); 52];
        let ranks = vec![Rank::Deuce,
                        Rank::Three,
                        Rank::Four,
                        Rank::Five,
                        Rank::Six,
                        Rank::Seven,
                        Rank::Eight,
                        Rank::Nine,
                        Rank::Ten,
                        Rank::Jack,
                        Rank::Queen,
                        Rank::King,
                        Rank::Ace];

        let suits = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

        for rank in &ranks {
            for suit in &suits {
                let card                 = Card::new(*rank, *suit);
                let ordinal              = card.ordinal;
                result[ordinal as usize] = card;
            }
        }
        result
    };
}

pub const CARD_COUNT: i32 = 52;

#[derive(Clone, Copy, Default, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub ordinal: i32,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
            ordinal: rank.ordinal() * 4 + suit.ordinal()
        }
    }
}
