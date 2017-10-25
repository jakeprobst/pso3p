
use player::PlayerId;
use pso3simulation::PSO3State;
use action::Action;
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use phase::pregamediscard::PreGameDiscard;

#[derive(Debug)]
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

        state.player1.deck.shuffle(&mut state.rng);
        state.player2.deck.shuffle(&mut state.rng);
        
        let mut actions = Vec::new();
        
        for _ in 0..5 {
            let p1card = state.player1.deck.draw();
            actions.push(StateChange::DrawCard {
                player: PlayerId::One,
                card: p1card.clone()
            });
            state.player1.hand.push(p1card);

            let p2card = state.player2.deck.draw();
            actions.push(StateChange::DrawCard {
                player: PlayerId::Two,
                card: p2card.clone()
            });
            state.player2.hand.push(p2card);
        }

        actions.push(StateChange::TurnOrder {
            player1_roll: p1roll,
            player2_roll: p2roll,
            active_player: active_player,
        });
        
        (actions, Some(Box::new(PreGameDiscard::new())))
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


