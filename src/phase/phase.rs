

use pso3simulation::PSO3State;
use action::Action;
use statechange::StateChange;
use error::SimulationError;
use player::PlayerId;

#[derive(Debug)]
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


pub fn is_active_player(action: &Action, playerid: &Option<PlayerId>) -> bool {
    if let Some(pid) = *playerid {
        match *action {
            Action::Player1(_) => PlayerId::One == pid,
            Action::Player2(_) => PlayerId::Two == pid,
        }
    }
    else {
        return false;
    }
}


pub trait Phase {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError>;
    fn phase_type(&self) -> PhaseType;
}

