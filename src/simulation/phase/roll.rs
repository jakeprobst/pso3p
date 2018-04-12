
use player::{PlayerId, Player};
use pso3simulation::PSO3State;
use action::{PlayerAction, Action};
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use phase::set::Set;
use std::cmp::{max, min};
use rand::{Rng, StdRng};

#[derive(Debug)]
pub struct Roll;

impl Roll {
    pub fn new() -> Roll {
        Roll {
        }
    }


    pub fn roll_atk_def(&self, state: &mut PSO3State) -> StateChange {
        let atk = state.rng.gen_range(1, 7);
        let def = state.rng.gen_range(1, 7);

        {
            let player = state.get_active_player();
            player.atk = atk;
            player.def = def;
        }
            
        StateChange::AtkDefDiceRoll {
            player: state.active_player,
            atk: atk,
            def: def,
        }
    }
}





/*
// TODO: actual random
fn roll_dice(id: PlayerId, rng: &mut StdRng, atk: &mut u8, def: &mut u8) -> (Vec<StateChange>, Option<Box<Phase + Send>>) {
    let d1 = rng.gen_range(1, 7);
    let d2 = rng.gen_range(1, 7);

    *atk = max(d1, d2);
    *def = min(d1, d2);

    let mut actions = Vec::new();
    actions.push(StateChange::AtkDefDiceRoll {
        player: id,
        atk: *atk,
        def: *def,
    });

    (actions, Some(Box::new(Set::new())))
}

fn handle_player_action(action: PlayerAction, rng: &mut StdRng, player: &mut Player)
                        -> Result<(Vec<StateChange>, Option<Box<Phase + Send>>), SimulationError> {
    match action {
        PlayerAction::RollDice => Ok(roll_dice(player.id, rng, &mut player.atk, &mut player.def)),
        _ => Err(SimulationError::InvalidAction(PhaseType::Roll, action))
    }
}

impl Phase for Roll {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase + Send>>), SimulationError> {
        if !is_active_player(&action, state.active_player) {
            return Err(SimulationError::NotYourTurn);
        }
        match action {
            Action::Player1(act) => handle_player_action(act, &mut state.rng, &mut state.player1),
            Action::Player2(act) => handle_player_action(act, &mut state.rng, &mut state.player2),
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::Roll
    }
}



*/
