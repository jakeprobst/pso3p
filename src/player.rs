

use card::Card;
use deck::Deck;


#[derive(Debug, Copy, Clone)]
pub enum PlayerId {
    One,
    Two,
}



#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub deck: Deck,
    experience: usize,
}






impl Player {
    pub fn new(deck: Deck) -> Player {
        Player {
            hand: Vec::new(),
            deck: deck,
            experience: 0,
        }
    }
}
