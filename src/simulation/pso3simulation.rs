


use action::Action;
use player::{Player, PlayerId};
use deck::Deck;
use field::Field;
use boardstate::BoardState;
//use phase;
//use phase::phase::Phase;
use phase::phase::Phase;
use phase::gamestart::GameStart;
use error::SimulationError;
use statechange::StateChange;
use rand::{StdRng, SeedableRng};


#[derive(Debug)]
pub struct PSO3State {
    pub rng: StdRng,
    pub boardstate: BoardState,
    pub active_player: PlayerId,
    pub player1: Player,
    pub player2: Player,
}







pub struct PSO3Simulation {
    phase: Box<Phase>,
    state: PSO3State,
}





impl PSO3Simulation {
    pub fn new(field: Field, mut p1deck: Deck, mut p2deck: Deck) -> PSO3Simulation {
        let mut rng =  StdRng::from_seed(&[1,2,3,4]);
        //let mut rng = StdRng::new().unwrap();
        p1deck.shuffle(&mut rng);
        p2deck.shuffle(&mut rng);
        PSO3Simulation {
            phase: Box::new(GameStart::new()),
            state: PSO3State {
                rng: rng,
                boardstate: BoardState::new(field, &p1deck.story_character, &p2deck.story_character),
                active_player: PlayerId::One,
                player1: Player::new(PlayerId::One, p1deck),
                player2: Player::new(PlayerId::Two, p2deck),
            }
        }
    }

    pub fn apply_action(&mut self, action: Action) -> Result<Vec<StateChange>, SimulationError> {
        let (mut statechange, newphase) = self.phase.handle_action(&mut self.state, action)?;
        if let Some(phase) = newphase {
            self.phase = phase;
            statechange.push(StateChange::PhaseChange(self.phase.phase_type()))
        }
        Ok(statechange)
    }
}
