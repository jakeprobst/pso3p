
use action::{Action, PlayerAction};
use phase::phase::PhaseType;


#[derive(Debug)]
pub enum SimulationError {
    //InvalidAction(PhaseType, Action),
    InvalidAction(PhaseType, PlayerAction),
    NotYourTurn,
}




