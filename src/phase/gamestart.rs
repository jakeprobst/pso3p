
use player::PlayerId;
use pso3simulation::PSO3State;
use action::Action;
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange, TurnOrder};
use phase::pregamediscard::PreGameDiscard;

pub struct GameStart;

impl GameStart {
    pub fn new() -> GameStart {
        GameStart {
        }
    }
    
    // TODO: actually random numbers
    fn roll_for_first_player(&mut self, state: &mut PSO3State) -> (Vec<StateChange>, Option<Box<Phase>>) {
        let p1roll = 5;
        let p2roll = 3;

        let active_player = if p1roll > p2roll {
            PlayerId::One
        }
        else {
            PlayerId::Two
        };

        state.active_player = Some(active_player);

        // TODO: fill hands

        
        (vec![StateChange::TurnOrder(TurnOrder {
            player1_roll: p1roll,
            player2_roll: p2roll,
            active_player: active_player,
        })],
         Some(Box::new(PreGameDiscard::new())))
    }
}


impl Phase for GameStart {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        match action {
            Action::RollForFirstPlayer => Ok(self.roll_for_first_player(state)),
            _ => Err(SimulationError::InvalidAction(self.phase_type(), action))
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::GameStart
    }
    
}


