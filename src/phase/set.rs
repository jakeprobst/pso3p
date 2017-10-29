



use player::Player;
use boardstate::BoardState;
use phase::phase::{Phase, PhaseType};
use action::{Action, PlayerAction};
use error::SimulationError;
use statechange::StateChange;
use pso3simulation::PSO3State;

pub struct Set;


impl Set {
    pub fn new() -> Set {
        Set {
        }
    }
}




fn handle_player_action(action: PlayerAction, player: &mut Player) -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
    match action {
        _ => Err(SimulationError::InvalidAction(PhaseType::Roll, action))
    }
}

impl Phase for Set {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        match action {
            Action::Player1(act) => handle_player_action(act, &mut state.player1),
            Action::Player2(act) => handle_player_action(act, &mut state.player2),
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::Set
    }
    
}
