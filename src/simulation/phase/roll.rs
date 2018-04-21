
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
        let d1 = state.rng.gen_range(1, 7);
        let d2 = state.rng.gen_range(1, 7);

        let atk = max(d1, d2);
        let def = min(d1, d2);

        {
            let player = state.get_active_player_mut();
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

