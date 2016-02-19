extern crate bit_vec;

use bit_vec::BitVec;
use structs::card::{Card, CARD_COUNT, CARD_VALUES};


#[derive(Clone, Debug)]
pub struct Hand {
    pub bit_set: BitVec,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            bit_set: BitVec::from_elem(CARD_COUNT as usize, false)
        }
    }

    pub fn plus_cards(&mut self, cards: [Card; 2]) {
        for card in cards.iter() {
            self.bit_set.set(card.ordinal as usize, true);
        }
    }

    pub fn minus_cards(&mut self, cards: [Card; 2]) {
        for card in cards.iter() {
            self.bit_set.set(card.ordinal as usize, false);
        }
    }

    pub fn plus_card(&mut self, card: Card) {
        self.bit_set.set(card.ordinal as usize, true);
    }

    pub fn minus_card(&mut self, card: Card) {
        self.bit_set.set(card.ordinal as usize, false);
    }

    pub fn num_of_cards(&self) -> usize {
        self.bit_set.iter().filter(|x| *x==true).count()
    }

    pub fn to_cards(&self) -> Vec<Card> {
        //TODO try to do it functional way
        /*
        let mut result: Vec<Card> = Vec::new();

        let mut iter = self.bit_set.iter();
        loop {
            match iter.position(|x| x == true) {
                Some(x) => {result.push(CARD_VALUES[x as usize].clone())},
                None => break,
            }
            match iter.next() {
                Some(_) => {},
                None => break,
            }
        }
        result
        */
        let mut result: Vec<Card> = Vec::new();
        for num in 0..self.bit_set.len() {
            if self.bit_set.get(num).unwrap() {
                result.push(CARD_VALUES[num]);
            }
        }
        result
    }

    pub fn contains(&self, card_ordinal: usize) -> bool {
        match self.bit_set.get(card_ordinal) {
            Some(x) => return x,
            None    => return false,
        }
    }
}
