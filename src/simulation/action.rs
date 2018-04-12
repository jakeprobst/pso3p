



//use card::CardId;
//use fieldobject::Position;


use pso3simulation::PSO3State;
use phase::phase::Phase;
use statechange::StateChange;

use std;

/*#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerAction {
    RollForFirst,
    FillHand,
    DiscardHand,
    KeepHand,
    RollDice,
    SetCard(CardId, Position),
    
    
    Discard(CardId),
    EndDiscard,
}

#[derive(Debug)]
pub enum Action {
    Player1(PlayerAction),
    Player2(PlayerAction),
}

 */


#[derive(Debug, Clone)]
pub enum PlayerAction {
}


pub trait Action: std::fmt::Debug {
    fn apply(&self, &mut Phase, &mut PSO3State) -> Vec<StateChange>;
    // should probably return a SimulationError
    fn is_valid(&self, &Phase, &PSO3State) -> bool;
}
