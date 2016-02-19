use structs::card::{Card, CARD_COUNT, CARD_VALUES};


pub const PAIR_COUNT: i32 = CARD_COUNT * (CARD_COUNT - 1) / 2;

lazy_static! {
    pub static ref PAIR_VALUES: [Pair; PAIR_COUNT as usize] = {
        let mut result: [Pair; PAIR_COUNT as usize] = [Pair::default(); PAIR_COUNT as usize];
        let mut ordinal_count                       = 0;

        for i in 0..CARD_COUNT {
            for j in (i+1)..CARD_COUNT {
                let pair                       = Pair::new(CARD_VALUES[i as usize], CARD_VALUES[j as usize], ordinal_count);
                result[ordinal_count as usize] = pair;
                ordinal_count += 1;
            }
        }
        result
    };

    pub static ref PAIR_VALUES_BY_CARD: [[Pair; CARD_COUNT as usize]; CARD_COUNT as usize] = {
        let mut result        = [[Pair::default(); CARD_COUNT as usize]; CARD_COUNT as usize];
        let mut ordinal_count = 0;

        for i in 0..CARD_COUNT {
            for j in (i+1)..CARD_COUNT {
                let pair = Pair::new(CARD_VALUES[i as usize], CARD_VALUES[j as usize], ordinal_count);
                result[i as usize][j as usize] = pair;
                result[j as usize][i as usize] = pair;
                ordinal_count += 1;
            }
        }
        result
    };

    pub static ref PAIR_INTERSECTS_PAIR: Box<[[bool; PAIR_COUNT as usize]; PAIR_COUNT as usize]> = {
        let mut result = box [[false; PAIR_COUNT as usize]; PAIR_COUNT as usize];

        for item_one in PAIR_VALUES.iter() {
            for item_two in PAIR_VALUES.iter() {
                if item_one.cards[0].ordinal == item_two.cards[0].ordinal ||
                   item_one.cards[0].ordinal == item_two.cards[1].ordinal ||
                   item_one.cards[1].ordinal == item_two.cards[0].ordinal ||
                   item_one.cards[1].ordinal == item_two.cards[1].ordinal {
                       result[item_one.ordinal as usize][item_two.ordinal as usize] = true;
               }
           }
       }
       result
   };

   pub static ref PAIR_INTERSECTS_CARD: Box<[[bool; CARD_COUNT as usize]; PAIR_COUNT as usize]> = {
        let mut result = box [[false; CARD_COUNT as usize]; PAIR_COUNT as usize];

        for pair in PAIR_VALUES.iter() {
            for card in CARD_VALUES.iter() {
                if pair.cards[0].ordinal == card.ordinal ||
                   pair.cards[1].ordinal == card.ordinal {
                       result[pair.ordinal as usize][card.ordinal as usize] = true;
                   }
            }
        }
        result
    };
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Pair {
    pub cards: [Card; 2],
    pub ordinal: i32,
}

impl Pair {
    pub fn new(card_one: Card, card_two: Card, ordinal: i32) -> Pair {
        Pair {
            cards: [card_one, card_two],
            ordinal: ordinal
        }
    }

    pub fn get(card_one: Card, card_two: Card) -> Pair {
        PAIR_VALUES_BY_CARD[card_one.ordinal as usize][card_two.ordinal as usize]
    }

    pub fn intersects_pair(&self, pair: Pair) -> bool {
        PAIR_INTERSECTS_PAIR[self.ordinal as usize][pair.ordinal as usize]
    }

    pub fn intersects_card(&self, card: Card) -> bool {
        PAIR_INTERSECTS_CARD[self.ordinal as usize][card.ordinal as usize]
    }
}
