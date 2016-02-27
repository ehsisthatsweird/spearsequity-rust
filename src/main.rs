#![feature(box_syntax)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]

extern crate lazy_static;
extern crate bit_vec;

mod structs;

use structs::{Card, Rank, Suit, CARD_VALUES, CARD_COUNT};
use structs::card;
use structs::{PAIR_VALUES, PAIR_INTERSECTS_PAIR, PAIR_INTERSECTS_CARD};
use structs::pair;
use structs::hand::Hand;
use structs::state_table_eval;
use structs::state_table_eval::KEYS;
use bit_vec::BitVec;


fn main() {
    /*
    println!("card count: {}", CARD_COUNT);

    println!("card values: {}", CARD_VALUES.len());

    print!("card ordinal: ");
    for item in CARD_VALUES.iter() {
        print!("{}, ", item.ordinal);
    }

    println!("pair values: {}", PAIR_VALUES.len());

    println!("pair pair intersections: {}", PAIR_INTERSECTS_PAIR.len());

    println!("pair card intersections: {}", PAIR_INTERSECTS_CARD.len());

    let mut yes = 0;
    let mut no  = 0;
    for item in PAIR_INTERSECTS_CARD.iter() {
        for x in item.iter() {
            if *x {
                yes += 1;
            } else {
                no += 1;
            }
        }
    }
    println!("yes {}, no {}", yes, no);

    */
    let card_one   = Card::new(Rank::Deuce, Suit::Spades);
    let card_two   = Card::new(Rank::Ace, Suit  ::Hearts);
    let card_three = Card::new(Rank::King, Suit ::Diamonds);
    let card_four  = Card::new(Rank::Seven, Suit::Clubs);

    let pair       = pair::Pair::get(card_one, card_two);
    let pair_two   = pair::Pair::get(card_two, card_three);
    let pair_three = pair::Pair::get(card_one, card_four);
    println!("{:?}", pair);

    println!("{:?} {:?}", pair.intersects_card(card_three), pair.intersects_card(card_one));
    println!("{:?} {:?}", pair.intersects_pair(pair_two), pair_two.intersects_pair(pair_three));

    let mut hand = Hand::new();
    hand.plus_card(Card::new(Rank::Eight, Suit::Spades));
    hand.plus_card(Card::new(Rank::Jack, Suit::Clubs));

    //println!("number of cards: {}", hand.num_of_cards());

    let test_vec = hand.to_cards();

    for item in test_vec {
        println!("{:?}", item);
    }

    println!("contains yes no {} {}", hand.contains(27), hand.contains(28));

    /*
    let test_suit: Suit = "c".into();
    println!("test suit: {:?}", test_suit);

    println!("{:#b}", state_table_eval::format_card_8bit((Card::new(Rank::Jack, Suit  ::Diamonds)).ordinal));

    println!("KEYS size: {:?}", unsafe {KEYS.len()});
    */
}

#[test]
fn test_card_count_number() {
    assert!(CARD_COUNT == 52);
}

#[test]
fn test_card_values_len() {
    assert!(CARD_VALUES.len() == 52);
}

#[test]
fn test_pair_values_len() {
    assert!(PAIR_VALUES.len() == 1326);
}

#[test]
fn test_pair_intersects_pair_len() {
    assert!(PAIR_INTERSECTS_PAIR.len() == 1326);
}

#[test]
fn test_pair_intersects_card_len() {
    assert!(PAIR_INTERSECTS_CARD.len() == 1326);
}

#[test]
fn test_test_card_intersection_number() {
    let mut yes = 0;
    let mut no  = 0;
    for item in PAIR_INTERSECTS_CARD.iter() {
        for x in item.iter() {
            if *x {
                yes += 1;
            } else {
                no += 1;
            }
        }
    }
    assert!(yes == 2652 && no == 66300);
}

#[test]
fn test_suit() {
    let test_suit: Suit = "c".into();
    assert!(test_suit.to_string() == "c");
}

#[test]
fn test_convert_card_to_8bit() {
    assert!(state_table_eval::format_card_8bit((Card::new(Rank::Jack, Suit  ::Diamonds)).ordinal) == 0b10100001);
}

#[test]
fn test_key_size() {
    assert!(unsafe{KEYS.len()} == 612978);
}

#[test]
fn test_plus_card() {
    let mut hand = Hand::new();
    hand.plus_card(Card::new(Rank::Eight, Suit::Spades));
    hand.plus_card(Card::new(Rank::Jack, Suit::Clubs));

    assert!(hand.num_of_cards() == 2);
}
