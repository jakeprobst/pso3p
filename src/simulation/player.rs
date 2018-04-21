

use card::{CardInstance, CardId};
use deck::Deck;


#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerId {
    #[serde(rename="1")]
    One,
    #[serde(rename="2")]
    Two,
}


/*#[derive(Debug)]
pub struct Hand {
    hand: Vec<CardInstance>,
}

impl Hand {
    fn new() -> Hand {
        Hand {
            hand: Vec::new()
        }
    }

    pub fn add(&mut self, card: CardInstance) {
        self.hand.push(card);
    }

    pub fn pop(&mut self, card: CardId) -> Option<CardInstance> {
        let cpos = self.hand.iter().position(|c| c.id() == card_id);
        let c = self.hand[cpos].clone();
        self.hand.remove(cpos);
        return c;
    }

    pub fn get(&mut self, card_id: CardId) -> Option<CardInstance> {
        self.hand.iter().find(|c| c.id() == card_id)
        /*let fcard = self.hand.iter().find(|c| c.id() == card_id);
        if let Some(card) = fcard {
            Some(card.clone())
        }
        else {
            None
        }*/
    }
}*/

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub hand: Vec<CardInstance>,
    //pub hand: Hand,
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

    pub fn draw_card(&mut self) -> CardInstance {
        let cardinst = CardInstance::new(self.deck.draw());
        self.hand.push(cardinst.clone());
        cardinst
    }

    pub fn get_card(&self, card_id: CardId) -> Option<&CardInstance> {
        self.hand.iter().find(|c| c.id() == card_id)
    }

    pub fn pop_card(&mut self, card_id: CardId) -> CardInstance {
        let cpos = self.hand.iter().position(|c| c.id() == card_id).unwrap();
        let c = self.hand[cpos].clone();
        self.deck.discard(c.card.clone());
        return c;
    }

    pub fn discard_card(&mut self, card: CardInstance) {
        self.hand.retain(|c| c.id() != card.id());
        self.deck.discard(card.card);
    }
}
