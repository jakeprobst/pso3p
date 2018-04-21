



use card::{Card, CharacterCard};
use rand::Rng;
use std::collections::VecDeque;



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
    story_character: Option<CharacterCard>,
    cards: VecDeque<Card>,
}


impl DeckBuilder {
    pub fn new() -> DeckBuilder {
        DeckBuilder {
            dtype: None,
            story_character: None,
            cards: VecDeque::new(),
        }
    }

    pub fn faction(mut self, dtype: DeckType) -> DeckBuilder {
        self.dtype = Some(dtype);
        self
    }
    
    pub fn character(mut self, sc: CharacterCard) -> DeckBuilder {
        self.story_character = Some(sc);
        self
    }
    
    pub fn card(mut self, card: Card) -> DeckBuilder {
        self.cards.push_back(card);
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
    pub story_character: CharacterCard,
    pub dtype: DeckType,
    cards: VecDeque<Card>,
}





impl Deck {
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R)  {
        // this is not defined for vecdeque
        //rng.shuffle(self.cards.as_mut_slice());
        for i in 0..self.cards.len()-1 {
            let r = rng.gen_range(i, self.cards.len());
            self.cards.swap(i, r);
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }

    pub fn discard(&mut self, card: Card) {
        self.cards.push_back(card);
    }
}
