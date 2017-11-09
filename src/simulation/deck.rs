



use card::{Card, CharacterCard};
use rand::Rng;



#[derive(Debug)]
pub enum DeckType {
    Hunter,
    Arkz,
}


#[derive(Debug)]
pub enum DeckError {
    InvalidCardCount,
    NoStoryCharacter,
    InvalidStoryCharacter(CharacterCard),
    InvalidCard(Card),
}

#[derive(Debug)]
pub struct DeckBuilder {
    dtype: Option<DeckType>,
    story_character: Option<Card>,
    cards: Vec<Card>,
}


impl DeckBuilder {
    pub fn new() -> DeckBuilder {
        DeckBuilder {
            dtype: None,
            story_character: None,
            cards: Vec::new(),
        }
    }

    pub fn faction(mut self, dtype: DeckType) -> DeckBuilder {
        self.dtype = Some(dtype);
        self
    }
    
    pub fn character(mut self, sc: Card) -> DeckBuilder {
        self.story_character = Some(sc);
        self
    }
    
    pub fn card(mut self, card: Card) -> DeckBuilder {
        self.cards.push(card);
        self
    }

    fn is_valid(&self) -> bool {
        true
    }
    
    pub fn deck(self) -> Result<Deck, DeckError> {
        Ok(Deck {
            dtype: self.dtype.unwrap(),
            story_character: self.story_character.unwrap(),
            cards: self.cards,
        })
    }
}

#[derive(Debug)]
pub struct Deck {
    pub story_character: Card,
    pub dtype: DeckType,
    cards: Vec<Card>,
}





impl Deck {
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R)  {
        rng.shuffle(self.cards.as_mut_slice());
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn discard(&mut self, card: Card) {
        self.cards.push(card);
    }
}
