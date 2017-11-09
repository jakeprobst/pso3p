
use pso3simulation::PSO3State;
use action::{Action, PlayerAction};
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange, };
use player::Player;
use phase::roll::Roll;
use card::CardInstance;

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

}

fn discard_hand(done: &mut bool, player: &mut Player) -> Vec<StateChange> {
    if *done == false {
        *done = true;
        
        let mut actions = Vec::new();
        for c in &player.hand {
            actions.push(StateChange::DiscardCard {
                player: player.id,
                card: c.id(),
            });
            player.deck.discard(c.card.clone());
        }
        player.hand = Vec::new();
        for _ in 0..5 {
            let cardinst = player.draw();
            actions.push(StateChange::DrawCard {
                player: player.id,
                card: cardinst,
            });
        }

        actions
    }
    else {
        Vec::new()
    }
}

fn keep_hand(done: &mut bool) -> Vec<StateChange> {
    if *done == false {
        *done = true;
    }
    Vec::new()
}

fn handle_player_action(action: PlayerAction, player: &mut Player, done: &mut bool)
                        -> Result<Vec<StateChange>, SimulationError> {
    match action {
        PlayerAction::DiscardHand => {
            Ok(discard_hand(done, player))
        }
        PlayerAction::KeepHand => {
            Ok(keep_hand(done))
        }
        _ => Err(SimulationError::InvalidAction(PhaseType::PreGameDiscard, action))
    }
}





impl Phase for PreGameDiscard {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        let actions = match action {
            Action::Player1(act) => handle_player_action(act, &mut state.player1, &mut self.p1done)?,
            Action::Player2(act) => handle_player_action(act, &mut state.player2, &mut self.p2done)?,
            
            //Action::DiscardHand(pid) => Ok(self.discard_hand(state, pid)),
            //Action::KeepHand(pid) => Ok(self.keep_hand(state, pid)),
        };
        Ok((actions, self.next_phase()))
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::PreGameDiscard
    }
}















