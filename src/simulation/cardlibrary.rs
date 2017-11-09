


use std::fs;
use std::io::prelude::*;

use card::Card;
use serde_yaml;
use std::error::Error;



pub struct CardLibrary {
    cards: Vec<Card>,
}


impl CardLibrary {
    pub fn new(path: &str) -> CardLibrary {
        let mut cards = Vec::new();
        //let cardfiles = fs::read_dir("./resources/cards/").unwrap();
        let cardfiles = fs::read_dir(path).unwrap();
        for card in cardfiles {
            let ucard = card.unwrap();
            let mut cfile = fs::File::open(ucard.path()).unwrap();
            let mut cdata = String::new();
            cfile.read_to_string(&mut cdata);
            
            let c: Card = match serde_yaml::from_str::<Card>(&cdata) {
                Ok(c) => c,
                Err(m) => {
                    panic!("could not load {}: {}", ucard.path().to_str().unwrap(), m.description())
                }
            };
            cards.push(c);
        }
        
        CardLibrary {
            cards: cards,
        }
    }

    pub fn get_by_id(&self, id: u32) -> Option<Card> {
        for c in &self.cards {
            match *c {
                Card::Character(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
                Card::Item(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
                Card::Monster(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
                Card::Action(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
                Card::Assist(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
                Card::Boss(ref c2) => {
                    if c2.id == id {
                        return Some(c.clone());
                    }
                }
            }
        }
        None
    }

}
