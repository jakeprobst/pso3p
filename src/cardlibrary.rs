


use std::fs;
use std::io::prelude::*;

use card::Card;
use serde_yaml;
use std::error::Error;



pub struct CardLibrary {
    cards: Vec<Card>,
}


impl CardLibrary {
    pub fn new() -> CardLibrary {
        let mut cards = Vec::new();
        let cardfiles = fs::read_dir("./resources/cards/").unwrap();
        for card in cardfiles {
            let ucard = card.unwrap();
            println!("{:?}", ucard);
            let mut cfile = fs::File::open(ucard.path()).unwrap();
            let mut cdata = String::new();
            cfile.read_to_string(&mut cdata);
            
            match serde_yaml::from_str::<Card>(&cdata) {
                Ok(_) => {},
                Err(m) => {
                    println!("    {:?},", m);
                    println!("    {},", m.description().split("`").nth(1).unwrap());
                }
            }
            //cards.push(c);
        }
        
        CardLibrary {
            cards: cards,
        }
    }
}
