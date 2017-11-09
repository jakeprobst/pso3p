#![feature(integer_atomics)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate rand;


mod action;
mod boardstate;
mod card;
mod deck;
mod player;
mod phase;
mod pso3simulation;
mod field;
mod fieldobject;
mod error;
mod statechange;
mod cardlibrary;

use action::{Action, PlayerAction};
//use player::Player;
use pso3simulation::PSO3Simulation;
use deck::{Deck, DeckBuilder, DeckType};
use field::Field;
use cardlibrary::CardLibrary;
use fieldobject::Position;


use card::*;
use std::fs;
use std::io::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct OldCard {
    num: u32,
    name: String,
    #[serde(rename = "type")]
    ttype: String,
    class: String,
    assisttime: i8,
    cost: u32,
    hp: i32,
    ap: i32,
    tp: i32,
    mv: i32,
    target: String,
    range: String,
    left: String,
    top: String,
    right: String,
    ability: Vec<String>,
    restrict: Vec<String>,
}


fn fix_target(s: &String) -> &'static str {
    match s.as_ref() {
        "single" => "Single",
        "multi" => "Multiple",
        "ally" => "Ally",
        "self" => "Self",
        "self once" => "Self",
        "selfally" => "Team",
        "team" => "Team",
        "everyone" => "Everyone",
        "all" => "Everyone",
        "assist card" => "Everyone",
        "unknown" => "Multiple",
        _ => "",
    }
}

fn fix_range(s: &String) -> Vec<String> {
    let mut r: Vec<String>= s.split("/").map(|a| a.replace("*", "+")).collect();
    r.reverse();
    r
}

fn fix_actionlink(s: &String) -> Vec<String> {
    s.split(",")
        .filter(|a| a.len() > 1)
        .map(|a| if a == "brown" {"Pollux"} else {a})
        .map(|a| if a == "???7" {"Castor"} else {a})
        .map(|a| if a == "???4" {"Leukon"} else {a})
        .map(|a| a.split_at(1))
        .map(|(a, b)| a.to_uppercase() + b)
        .collect()
}

fn fix_ability(s: &String) -> String {
    if s == "Action" { // PArms Blade +
        "ActionDisrupter".to_string()
    }
    else {
        s.replace(" ", "")
            .replace("-", "")
            .replace(".", "")
            .replace("/", "")
            .replace("=", "")
            .replace("+", "")
            .replace(":", "")
            .replace("'", "")
            .replace("&", "")
    }
}

fn write_array<R: std::fmt::Display>(of: &mut fs::File, label: &str, arr: Vec<R>) {
    if arr.len() == 0 {
        writeln!(of, "  {}: []", label);
    }
    else {
        writeln!(of, "  {}:", label);
        for r in arr {
            writeln!(of, "    - {}", r);
        }
    }
}


fn convert_cards() {
    let cardfiles = fs::read_dir("./oldstuff/rawcards/").unwrap();

    for card in cardfiles {
        let c2 = card.unwrap();
        println!("{:?}", c2);
        let mut cfile = fs::File::open(c2.path()).unwrap();
        let mut cdata = String::new();
        cfile.read_to_string(&mut cdata);
        
        let c: OldCard = serde_json::from_str(&cdata).unwrap();

        /*if c.ttype == "Boss" {
            continue;
        }*/
        
        let mut of = fs::File::create(format!("resources/cards/{}", c2.file_name().into_string().unwrap())).unwrap();
        match c.ttype.as_ref() {
            "Hunter" | "Arkz" => {
                writeln!(of, "Character:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  type: {}", c.ttype);
                writeln!(of, "  class: {}", c.class);
                writeln!(of, "  hp: {}", c.hp);
                writeln!(of, "  ap: {}", c.ap);
                writeln!(of, "  tp: {}", c.tp);
                writeln!(of, "  mv: {}", c.mv);
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "range", fix_range(&c.range));
                write_array(&mut of, "toplink", fix_actionlink(&c.top));
                write_array(&mut of, "rightlink", fix_actionlink(&c.right));
                write_array(&mut of, "ability", c.ability.iter().map(|a| fix_ability(&a)).collect());
            }
            "Item" => {
                writeln!(of, "Item:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  type: {}", c.class.replace("Guard", "Shield"));
                writeln!(of, "  cost: {}", c.cost);
                writeln!(of, "  hp: {}", c.hp);
                writeln!(of, "  ap: {}", c.ap);
                writeln!(of, "  tp: {}", c.tp);
                writeln!(of, "  mv: {}", c.mv);
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "range", fix_range(&c.range));
                write_array(&mut of, "toplink", fix_actionlink(&c.top));
                write_array(&mut of, "rightlink", fix_actionlink(&c.right));
                write_array(&mut of, "ability", c.ability.iter()
                            .filter(|a| a != &&"Tech X2".to_string()) // nei's claw
                            .map(|a| fix_ability(&a))
                            .collect());
            }
            "Creature" => {
                writeln!(of, "Monster:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  type: {}", c.class.replace(".", ""));
                writeln!(of, "  cost: {}", c.cost);
                writeln!(of, "  hp: {}", c.hp);
                writeln!(of, "  ap: {}", c.ap);
                writeln!(of, "  tp: {}", c.tp);
                writeln!(of, "  mv: {}", c.mv);
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "range", fix_range(&c.range));
                write_array(&mut of, "toplink", fix_actionlink(&c.top));
                write_array(&mut of, "rightlink", fix_actionlink(&c.right));
                write_array(&mut of, "ability", c.ability.iter().map(|a| fix_ability(&a)).collect());
            }
            "Action" => {
                writeln!(of, "Action:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  type: {}", c.class.replace(" ", ""));
                writeln!(of, "  cost: {}", c.cost);
                writeln!(of, "  hp: {}", if c.hp == 999 {255} else if c.hp < 0 {0} else {c.hp});
                writeln!(of, "  ap: {}", if c.ap == 999 {255} else {c.ap});
                writeln!(of, "  tp: {}", c.tp);
                writeln!(of, "  mv: {}", c.mv);
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "range", fix_range(&c.range));
                write_array(&mut of, "leftlink", fix_actionlink(&c.left));
                write_array(&mut of, "toplink", fix_actionlink(&c.top));
                write_array(&mut of, "rightlink", fix_actionlink(&c.right));
                write_array(&mut of, "ability", c.ability.iter().map(|a| fix_ability(&a)).collect());
            }
            "Assist" => {
                writeln!(of, "Assist:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  cost: {}", c.cost);
                writeln!(of, "  time: {}", if c.assisttime == -1 { 255 } else { c.assisttime as u8});
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "ability", c.ability.iter().map(|a| fix_ability(&a)).collect());
            }

            "Boss" => {
                writeln!(of, "Boss:");
                writeln!(of, "  id: {}", c.num);
                writeln!(of, "  name: {}", c.name);
                writeln!(of, "  hp: {}", c.hp);
                writeln!(of, "  ap: {}", c.ap);
                writeln!(of, "  tp: {}", c.tp);
                writeln!(of, "  mv: {}", c.mv);
                writeln!(of, "  target: {}", fix_target(&c.target));
                write_array(&mut of, "range", fix_range(&c.range));
                write_array(&mut of, "toplink", fix_actionlink(&c.top));
                write_array(&mut of, "rightlink", fix_actionlink(&c.right));
                write_array(&mut of, "ability", c.ability.iter()
                            .filter(|a| a != &&"Steady Damage-mod".to_string()) // Castor
                            .map(|a| fix_ability(&a))
                            .collect());
            }
            _ => panic!("bad card type")
        }









        
    }

}



fn main() {
    //println!("Hello, world!");



    //let player1 = Player::new();
    //let player2 = Player::new();

    

    //convert_cards();


    //println!("-----------");
    let card_library = CardLibrary::new();


    //let deck1 = Deck::new(DeckType::Hunter);
    //let deck2 = Deck::new(DeckType::Arkz);

    println!("{:?}", card_library.get_by_id(1).unwrap());


    let mut db = DeckBuilder::new()
        .faction(DeckType::Hunter)
        .character(card_library.get_by_id(1).unwrap());
    
    for c in vec![9, 12, 22, 23, 40, 44, 371, 197, 253, 246] {
        for _ in 0..3 {
            db = db.card(card_library.get_by_id(c).unwrap());
        }
    }
    let deck1 = db.deck().unwrap();


    let mut db = DeckBuilder::new()
        .faction(DeckType::Hunter)
        .character(card_library.get_by_id(2).unwrap());
    
    for c in vec![12, 22, 52, 380, 381, 382, 632, 197, 253, 246] {
        for _ in 0..3 {
            db = db.card(card_library.get_by_id(c).unwrap());
        }
    }
    let deck2 = db.deck().unwrap();    
    
    let mut sim = PSO3Simulation::new(Field::new(), deck1, deck2);
    println!("{:#?}", sim.apply_action(Action::Player1(PlayerAction::RollForFirst)));
    println!("{:#?}", sim.apply_action(Action::Player2(PlayerAction::RollForFirst)));
    println!("{:#?}", sim.apply_action(Action::Player1(PlayerAction::KeepHand)));
    println!("{:#?}", sim.apply_action(Action::Player2(PlayerAction::DiscardHand)));
    println!("{:#?}", sim.apply_action(Action::Player1(PlayerAction::RollDice)));
    println!("{:#?}", sim.apply_action(Action::Player1(PlayerAction::SetCard(2, Position::new(0, 0)))));

    /*let a = Card::Character(CharacterCard {
        num:  1,
        name: "Orland".to_string(),
        ctype: CharacterType::Hunter,
        class: CharacterClass::HUmar,
        hp: 0,
        ap: 1,
        tp: 0,
        mv: 3,
        target: TargetType::Single,
        range: Range::new(),
        toplink: vec![ActionLink::Red],
        rightlink: vec![ActionLink::Red, ActionLink::Blue, ActionLink::Yellow, ActionLink::Orange, ActionLink::Purple],
        ability: vec![],
    });


    if let Ok(z) = serde_yaml::to_string(&a) {
        println!("[{}]", z);
    }*/





    
        
    
}
