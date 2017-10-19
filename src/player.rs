

use card::Card;
use deck::Deck;


#[derive(Copy, Clone)]
pub enum PlayerId {
    One,
    Two,
}




pub struct Player {
    hand: Vec<Card>,
    deck: Deck,
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
