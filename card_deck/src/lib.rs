 use rand::Rng;
 #[derive(PartialEq,Debug)]
pub enum Suit {
    Heart,
    Diamonds,
    Spade,
    Club
   
}
 #[derive(PartialEq,Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        use Suit::*;
        match rand::thread_rng().gen_range(1..=4){
            1=>Heart,
            2=>Diamonds,
            3=>Spade,
            _=>Club
        }
    }

    pub fn translate(value: u8) -> Suit {
        use Suit::*;
        match value{
            1=>Heart,
            2=>Diamonds,
            3=>Spade,
            _=>Club
        }
    }
}


impl Rank {
    pub fn random() -> Rank {
        use Rank::*;
        match rand::thread_rng().gen_range(1..=13) {
            1=>Ace,
            11=>Jack,
            12=>Queen,
            13=>King,
            n=>Number(n)
        }
    }

    pub fn translate(value: u8) -> Rank {
        use Rank::*;
        match value{
             1=>Ace,
            11=>Jack,
            12=>Queen,
            13=>King,
            n=>Number(n)
        }
    }
}
#[derive(PartialEq,Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit==Suit::Spade && card.rank==Rank::Ace
}
