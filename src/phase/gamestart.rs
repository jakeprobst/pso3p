
use player::PlayerId;
use pso3simulation::PSO3State;
use action::{Action, PlayerAction};
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use phase::pregamediscard::PreGameDiscard;
use card::CardInstance;

use rand::Rng;

#[derive(Debug)]
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
    
}


// TODO: deal with p2 going first if both roll the same number
fn roll_for_first(proll: &mut Option<u8>, state: &mut PSO3State) {
    if *proll == None {
        *proll = Some(state.rng.gen_range(1, 7));
    }
}


fn post_roll(gamestart: &GameStart, state: &mut PSO3State) -> (Vec<StateChange>, Option<Box<Phase>>) {
    if let (Some(p1roll), Some(p2roll)) = (gamestart.p1roll, gamestart.p2roll) {
        let active_player = if p1roll > p2roll {
            PlayerId::One
        }
        else {
            PlayerId::Two
        };

        state.active_player = Some(active_player);

        let mut actions = Vec::new();
        
        for _ in 0..5 {
            let p1cardinst = state.player1.draw();
            actions.push(StateChange::DrawCard {
                player: PlayerId::One,
                card: p1cardinst,
            });

            let p2cardinst = state.player2.draw();
            actions.push(StateChange::DrawCard {
                player: PlayerId::Two,
                card: p2cardinst,
            });
        }

        actions.push(StateChange::TurnOrderRolls {
            player1_roll: p1roll,
            player2_roll: p2roll,
        });
        actions.push(StateChange::TurnChange {
            player: active_player,
        });
        
        (actions, Some(Box::new(PreGameDiscard::new())))
    }
    else {
       (Vec::new(), None)
    }
}

fn handle_player_action(action: PlayerAction, proll: &mut Option<u8>, state: &mut PSO3State)
                        -> Result<(), SimulationError> {
    match action {
        PlayerAction::RollForFirst => {
            roll_for_first(proll, state);
            Ok(())
        }
        _ => Err(SimulationError::InvalidAction(PhaseType::GameStart, action))
    }
    
}

impl Phase for GameStart {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        match action {
            Action::Player1(act) => {
                handle_player_action(act, &mut self.p1roll, state)?;
                Ok(post_roll(self, state))
            }
            Action::Player2(act) => {
                handle_player_action(act, &mut self.p2roll, state)?;
                Ok(post_roll(self, state))
            }
        }
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::GameStart
    }
}    

