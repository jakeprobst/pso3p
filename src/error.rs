
use action::Action;
use phase::phase::PhaseType;


#[derive(Debug)]
pub enum SimulationError {
    InvalidAction(PhaseType, Action),
}




