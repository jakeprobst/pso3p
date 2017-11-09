
extern crate pso3p;


use pso3p::action::{Action, PlayerAction};
use pso3p::pso3simulation::PSO3Simulation;
use pso3p::deck::{DeckBuilder, DeckType};
use pso3p::field::Field;
use pso3p::cardlibrary::CardLibrary;
use pso3p::fieldobject::Position;







fn main() {
    let card_library = CardLibrary::new("./resources/cards/");

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



    
}
