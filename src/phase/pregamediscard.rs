
use pso3simulation::PSO3State;
use action::Action;
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange, };
use player::PlayerId;
use phase::roll::Roll;


#[derive(Debug)]
pub struct PreGameDiscard {
    p1done: bool,
    p2done: bool,
}


impl PreGameDiscard {
    pub fn new() -> PreGameDiscard {
        PreGameDiscard {
            p1done: false,
            p2done: false,
        }
    }

    fn next_phase(&self) -> Option<Box<Phase>> {
        if self.p1done && self.p2done {
            Some(Box::new(Roll::new()))
        }
        else {
            None
        }
    }

    fn discard_hand(&mut self, state: &mut PSO3State, pid: PlayerId) -> (Vec<StateChange>, Option<Box<Phase>>) {
        match pid {
            PlayerId::One => {
                if self.p1done == false {
                    self.p1done = true;
                    // TODO: discard hand
                    (vec![StateChange::NoOp], None)
                }
                else {
                    (vec![StateChange::NoOp], None)
                }
            }
            PlayerId::Two => {
                if self.p2done == false {
                    self.p2done = true;
                    // TODO: discard hand
                    (vec![StateChange::NoOp], None)
                }
                else {
                    (vec![StateChange::NoOp], None)
                }
            }
        }
    }

    fn keep_hand(&mut self, state: &mut PSO3State, pid: PlayerId) -> (Vec<StateChange>, Option<Box<Phase>>) {
        match pid {
            PlayerId::One => {
                if self.p1done == false {
                    self.p1done = true;
                    // TODO: keep hand
                    (vec![StateChange::NoOp], None)
                }
                else {
                    (vec![StateChange::NoOp], None)
                }
            }
            PlayerId::Two => {
                if self.p2done == false {
                    self.p2done = true;
                    // TODO: keep hand
                    (vec![StateChange::NoOp], None)
                }
                else {
                    (vec![StateChange::NoOp], None)
                }
            }
        }
    }
}




impl Phase for PreGameDiscard {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {


        match action {
            Action::DiscardHand(pid) => Ok(self.discard_hand(state, pid)),
            Action::KeepHand(pid) => Ok(self.keep_hand(state, pid)),
            _ => Err(SimulationError::InvalidAction(PhaseType::PreGameDiscard, action))
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::PreGameDiscard
    }
}















