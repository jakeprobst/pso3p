
use action::Action;
use phase::phase::PhaseType;


pub enum SimulationError {
    InvalidAction(PhaseType, Action),
}




