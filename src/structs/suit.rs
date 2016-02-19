#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    //Error
}

impl Default for Suit {
    fn default() -> Self {
        Suit::Clubs
    }
}

impl From<&'static str> for Suit {
    fn from(string: &str) -> Self {
        match string {
            "c" => Suit::Clubs,
            "d" => Suit::Diamonds,
            "h" => Suit::Hearts,
            "s" => Suit::Spades,
            //_   => Suit::Error
            _   => panic!("Wrong Suit!")
        }
    }
}

impl Suit {
    pub fn new(string: &str) -> Suit {
        match string {
            "c" => Suit::Clubs,
            "d" => Suit::Diamonds,
            "h" => Suit::Hearts,
            "s" => Suit::Spades,
            //_   => Suit::Error
            _   => panic!("Wrong Suit!")
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            &Suit::Clubs    => "c",
            &Suit::Diamonds => "d",
            &Suit::Hearts   => "h",
            &Suit::Spades   => "s",
            //&Suit::Error    => "Err"
        }
    }
    /*
    pub fn is_valid(&self) -> bool {
        match self {
            &Suit::Error => false,
            _            => true
        }
    }
    */
    pub fn ordinal(&self) -> i32 {
        match self {
            &Suit::Clubs    => 0,
            &Suit::Diamonds => 1,
            &Suit::Hearts   => 2,
            &Suit::Spades   => 3,
            //&Suit::Error    => -1
        }
    }
}

pub fn get_suit(string: &str) -> Suit {
    match string {
        "c" => Suit::Clubs,
        "d" => Suit::Diamonds,
        "h" => Suit::Hearts,
        "s" => Suit::Spades,
        //_   => Suit::Error
        _   => panic!("Wrong Suit!")
    }
}
