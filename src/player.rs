

use card::CardInstance;
use deck::Deck;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerId {
    One,
    Two,
}



#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub hand: Vec<CardInstance>,
    pub deck: Deck,
    pub atk: u8,
    pub def: u8,
    experience: usize,
}






impl Player {
    pub fn new(id: PlayerId, deck: Deck) -> Player {
        Player {
            id: id,
            hand: Vec::new(),
            deck: deck,
            atk: 0,
            def: 0,
            experience: 0,
        }
    }

    pub fn draw(&mut self) -> CardInstance {
        let cardinst = CardInstance::new(self.deck.draw());
        self.hand.push(cardinst.clone());
        cardinst
    }
}
