

use card::{CardInstance, CardId};
use deck::Deck;


#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerId {
    #[serde(rename="1")]
    One,
    #[serde(rename="2")]
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

    pub fn hand_card_by_id(&mut self, card_id: CardId) -> Option<CardInstance> {
        let fcard = self.hand.iter().find(|c| c.id() == card_id);
        if let Some(card) = fcard {
            Some(card.clone())
        }
        else {
            None
        }
    }

    pub fn hand_discard_card(&mut self, card: CardInstance) {
        self.hand.retain(|c| c.id() != card.id());
        self.deck.discard(card.card);
    }
}
