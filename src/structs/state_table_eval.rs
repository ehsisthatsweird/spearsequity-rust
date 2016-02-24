use structs::products;
use structs::flushes;
use structs::unique;
use structs::values::VALUES;
use structs::hand::Hand;
use structs::card::Card;


pub const HAND_RANKS_SIZE: i32 = 32487834;
pub const OFFSET: [i32;9] = [0, 1277, 4137, 4995, 5853, 5863, 7140, 7296, 7452];
pub const LOOKUP_TABLE_SEZE: i32 = 612978;
const NUM_SUITS: i32 = 4;
const NUM_RANKS: i32 = 13;

//TODO find out array size
static mut HANDS: [i32; 999] = [0; 999];
static mut HAND_RANKS: [i32; HAND_RANKS_SIZE as usize] = [0; HAND_RANKS_SIZE as usize];
pub static mut NUM_KEYS: i32 = 1;
pub static mut MAX_KEY: i64 = 0;
pub static mut KEYS: [i64; LOOKUP_TABLE_SEZE as usize] = [0; LOOKUP_TABLE_SEZE as usize];


pub fn insert_key(key: i64) -> i32 {
    let mut result: i32 = 0;

    if key == 0 {
        return 0;
    }

    if key >= unsafe {MAX_KEY} {
        if key > unsafe {MAX_KEY} {
            unsafe {
                KEYS[NUM_KEYS as usize] = key;
                NUM_KEYS += 1;
                MAX_KEY = key;
            }
        }
        result = unsafe {NUM_KEYS - 1};
        return result;
    }

    let mut low = -1;
    let mut high = unsafe {NUM_KEYS};
    let mut pivot: i32;
    let mut difference: i64;

    while (high - low) > 1 {
        pivot = (low + high) >> 1;
        difference = unsafe {KEYS[pivot as usize] - key};

        if difference > 0 {
            high = pivot;
        } else if difference < 0 {
            low = pivot;
        } else {
            return pivot; //key already exists
        }
    }

    //key doesn't exist, insert key
    //TODO: why copyarray in java?
    unsafe {
        KEYS[high as usize] = key;
        NUM_KEYS += 1;
    }

    high
}
// 8-Bit Packed EquityRunner.Card Representation
// +--------+
// |rrrr--ss|
// +--------+
// r = rank of card		(deuce = 1, trey = 2, four = 3, five = 4,..., ace = 13)
// s = suit of card		(suits are arbitrary, can take value from 0 to 3)
//TODO: maybe change return type to u8
//TODO: check if the right return is 0..3 or 1..4
pub fn format_card_8bit(card: i32) -> i32 {
    //let result = card - 1;
    (((card >> 2) + 1) << 4) + (card & 3)
    // (((card >> 2) + 1) << 4) + (card & 3) + 1;
}

fn swap_card(card_one: i32, card_two: i32) {
    unsafe {
        if HANDS[card_one as usize] < HANDS[card_two as usize] {
            HANDS[card_one as usize] = HANDS[card_one as usize] ^ HANDS[card_two as usize];
            HANDS[card_two as usize] = HANDS[card_two as usize] ^ HANDS[card_one as usize];
            HANDS[card_one as usize] = HANDS[card_one as usize] ^ HANDS[card_two as usize];
        }
    }
}

pub fn sort_hand() {
    swap_card(0, 4);
    swap_card(1, 5);
    swap_card(2, 6);
    swap_card(0, 2);
    swap_card(1, 3);
    swap_card(4, 6);
    swap_card(2, 4);
    swap_card(3, 5);
    swap_card(0, 1);
    swap_card(2, 3);
    swap_card(4, 5);
    swap_card(1, 4);
    swap_card(3, 6);
    swap_card(1, 2);
    swap_card(3, 4);
    swap_card(5, 6);
}

pub fn get_index(key: i32) -> i32 {
    let result = -1;

    let mut low = -1;
    let mut high = 4888;
    let mut pivot;

    while (high - low) > 1 {
        pivot = (high + low) >> 1;
        if products::PRODUCTS[pivot as usize] > key {
            high = pivot;
        } else if products::PRODUCTS[pivot as usize] < key {
            low = pivot;
        } else {
            return pivot;
        }
    }

    result
}

//TODO: in java s is i16?
pub fn eval_5hand(c1: i32, c2: i32, c3:i32, c4:i32, c5:i32) -> i32 {
    let mut q = (c1 | c2 | c3 | c4 | c5) >> 16;
    // let mut s: i32;

    if (c1 & c2 & c3 & c4 & c5 & 0xF000) != 0 {
        return flushes::FLUSHES[q as usize];
    } else if unique::UNIQUE[q as usize] != 0 {
        // s = unique::UNIQUE[q as usize];
        // return s;
        return unique::UNIQUE[q as usize];
    } else {
        q = (c1 & 0xFF) * (c2 & 0xFF) * (c3 & 0xFF) * (c4 & 0xFF) * (c5 & 0xFF);
        q = get_index(q);

        return VALUES[q as usize];
    }
}

pub fn get_rank_from_hand(seven_card_hand: Hand) -> i32 {
    let cards = seven_card_hand.to_cards();
    let mut rank = 53;

    for item in cards {
        let c = item.ordinal + 1;
        unsafe {
            rank = HAND_RANKS[(c + rank) as usize];
        }
    }

    let rank_type = (rank >> 12) - 1;
    rank = rank & 0xFFF;

    OFFSET[rank_type as usize] + rank - 1
}

pub fn get_rank_from_cards(cards: Vec<Card>) -> i32 {
    let mut rank = 53;

    for item in cards {
        let c = item.ordinal + 1;
        unsafe {
            rank = HAND_RANKS[(c + rank) as usize];
        }
    }

    let rank_type = (rank >> 12) - 1;
    rank = rank & 0xFFF;

    OFFSET[rank_type as usize] + rank - 1
}
