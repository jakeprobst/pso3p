



use card::{Card, CharacterCard};





pub struct Deck {
    //character: CharacterCard,
    cards: Vec<Card>,
    
}





impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }
}
