

use pso3simulation::PSO3State;
use action::Action;
use statechange::StateChange;
use error::SimulationError;

pub enum PhaseType {
    GameStart,
    PreGameDiscard,
    Roll,
    Set,
    Move,
    Attack,
    Defend,
    Draw,
}


//pub struct PhaseType<'a>(&'a str);


pub trait Phase {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError>;
    fn phase_type(&self) -> PhaseType;
}

