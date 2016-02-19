#[derive(Clone, Copy, Debug)]
pub enum Rank {
    Deuce,
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
    Ace,
    //Error
}

impl Default for Rank {
    fn default() -> Self {
        Rank::Deuce
    }
}

impl From<&'static str> for Rank {
    fn from(string: &str) -> Self {
        match string {
            "2" => Rank::Deuce,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            //_   => Rank::Error
            _   => panic!("Wrong Rank!")
        }
    }
}

impl Rank {
    pub fn new(string: &str) -> Rank {
        match string {
            "2" => Rank::Deuce,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "T" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            //_   => Rank::Error
            _   => panic!("Wrong Rank!")
        }
    }
    pub fn to_string(&self) -> &str {
        match self {
            &Rank::Deuce => "2",
            &Rank::Three => "3",
            &Rank::Four  => "4",
            &Rank::Five  => "5",
            &Rank::Six   => "6",
            &Rank::Seven => "7",
            &Rank::Eight => "8",
            &Rank::Nine  => "9",
            &Rank::Ten   => "T",
            &Rank::Jack  => "J",
            &Rank::Queen => "Q",
            &Rank::King  => "K",
            &Rank::Ace   => "A",
            //&Rank::Error => "Err"
        }
    }
    /*
    pub fn is_valid(&self) -> bool {
        match self {
            &Rank::Error => false,
            _            => true
        }
    }
    */
    pub fn ordinal(&self) -> i32 {
        match self {
            &Rank::Deuce => 0,
            &Rank::Three => 1,
            &Rank::Four  => 2,
            &Rank::Five  => 3,
            &Rank::Six   => 4,
            &Rank::Seven => 5,
            &Rank::Eight => 6,
            &Rank::Nine  => 7,
            &Rank::Ten   => 8,
            &Rank::Jack  => 9,
            &Rank::Queen => 10,
            &Rank::King  => 11,
            &Rank::Ace   => 12,
            //&Rank::Error => -1
        }
    }
}

pub fn to_string(string: &str) -> Rank {
    match string {
        "2" => Rank::Deuce,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "T" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        //_   => Rank::Error
        _   => panic!("Wrong Rank!")
    }
}
