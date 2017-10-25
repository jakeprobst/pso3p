
use player::PlayerId;
use pso3simulation::PSO3State;
use action::Action;
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use phase::pregamediscard::PreGameDiscard;

#[derive(Debug)]
pub struct Roll;

impl Roll {
    pub fn new() -> Roll {
        Roll {
        }
    }
}

impl Phase for Roll {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        match action {
            _ => Err(SimulationError::InvalidAction(self.phase_type(), action))
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::Roll
    }
    
}



