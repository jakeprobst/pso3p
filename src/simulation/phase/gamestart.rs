
use player::PlayerId;
use pso3simulation::PSO3State;
use action::{Action, PlayerAction};
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use phase::pregamediscard::PreGameDiscard;
use card::CardInstance;

use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameStart {
    p1roll: Option<u8>,
    p2roll: Option<u8>,
}

impl GameStart {
    pub fn new() -> GameStart {
        GameStart {
            p1roll: None,
            p2roll: None,
        }
    }

    pub fn both_rolled(&self) -> bool {
        self.p1roll != None && self.p2roll != None
    }
}


#[derive(Debug)]
pub struct RollForFirst {
    player: PlayerId,
}

impl RollForFirst {
    pub fn new(p: PlayerId) -> RollForFirst {
        RollForFirst {
            player: p,
        }
    }
}

impl Action for RollForFirst {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        if let Phase::GameStart(gamestart) = phase {
            if self.player == PlayerId::One {
                gamestart.p1roll = Some(state.rng.gen_range(1, 7));
            }
            else if self.player == PlayerId::Two {
                gamestart.p2roll = Some(state.rng.gen_range(1, 7));
            }
            if let (Some(p1roll), Some(p2roll)) = (gamestart.p1roll, gamestart.p2roll) {
                let active_player = if p1roll > p2roll {
                    PlayerId::One
                }
                else {
                    PlayerId::Two
                };

                state.active_player = active_player;

                let mut actions = Vec::new();
                actions.push(StateChange::TurnOrderRolls {
                    player1_roll: p1roll,
                    player2_roll: p2roll,
                });
                actions.push(StateChange::TurnChange {
                    player: active_player,
                });

                return actions;
            }
        }
        Vec::new()
    }
    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        if let Phase::GameStart(gamestart) = phase {
            if (self.player == PlayerId::One && gamestart.p1roll != None) || (self.player == PlayerId::Two && gamestart.p2roll != None) {
                return false;
            }
        }
        else {
            return false;
        }

        return true;
    }
}
