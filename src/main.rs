


mod action;
mod boardstate;
mod card;
mod deck;
mod player;
mod phase;
mod pso3simulation;
mod field;
mod error;
mod statechange;

use action::Action;
use player::Player;
use pso3simulation::PSO3Simulation;
use deck::Deck;
use field::Field;






fn main() {
    //println!("Hello, world!");



    //let player1 = Player::new();
    //let player2 = Player::new();

    

    let mut sim = PSO3Simulation::new(Field::new(), Deck::new(), Deck::new());
    sim.apply_action(Action::RollForFirstPlayer);






    
        
    
}
