


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
use statechange::{StateChange};
use rand::{StdRng, SeedableRng};


#[derive(Debug)]
pub struct PSO3State {
    pub rng: StdRng,
    pub boardstate: BoardState,
    pub active_player: PlayerId,
    pub player1: Player,
    pub player2: Player,
}

impl PSO3State {
    pub fn get_player(&mut self, player: PlayerId) -> &mut Player {
        match player {
            PlayerId::One => &mut self.player1,
            PlayerId::Two => &mut self.player2,
        }
    }

    pub fn get_active_player(&mut self) -> &mut Player {
        let ap = self.active_player; // shut up the borrow checker
        self.get_player(ap)
    }
    
}





#[derive(Debug)]
pub struct PSO3Simulation {
    //phase: Box<Phase + Send>,
    //phase: Box<Phase>,
    pub phase: Phase,
    pub state: PSO3State,
}





impl PSO3Simulation {
    pub fn new(mut rng: StdRng, field: Field, mut p1deck: Deck, mut p2deck: Deck) -> PSO3Simulation {
        //let mut rng =  StdRng::from_seed(&[1,2,3,4]);
        //let mut rng = StdRng::new().unwrap();
        p1deck.shuffle(&mut rng);
        p2deck.shuffle(&mut rng);
        PSO3Simulation {
            phase: Phase::GameStart(GameStart::new()),
            //phase: Box::new(GameStart::new()),
            state: PSO3State {
                rng: rng,
                boardstate: BoardState::new(field, &p1deck.story_character, &p2deck.story_character),
                active_player: PlayerId::One,
                player1: Player::new(PlayerId::One, p1deck),
                player2: Player::new(PlayerId::Two, p2deck),
            }
        }
    }

    /*pub fn apply_action(&mut self, action: Action) -> Result<Vec<StateChange>, SimulationError> {
        let (mut statechange, newphase) = self.phase.handle_action(&mut self.state, action)?;
        if let Some(phase) = newphase {
            self.phase = phase;
            statechange.push(StateChange::PhaseChange(self.phase.phase_type()))
        }
        Ok(statechange)
}*/

    
    pub fn apply_action(&mut self, action: &Action) -> Result<Vec<StateChange>, SimulationError> {
        let mut state_change = Vec::new();
        if action.is_valid(&self.phase, &self.state) {
            state_change.append(&mut action.apply(&mut self.phase, &mut self.state))
        }
        else {
            
        }

        /*self.phase = if let Some(next_phase) = self.phase.next_phase() {
            state_change.append(&mut next_phase.on_start(&mut self.state));
            next_phase
        }
        else {
            self.phase
    };*/
        
        /*let next_phase = self.phase.next_phase();
        if let Some(mut phase) = next_phase {
            state_change.append(&mut phase.on_start(&mut self.state));
            self.phase = phase;
            //next_phase = Some(phase);
    }*/

        while let Some(mut phase) = self.phase.next_phase() {
            state_change.append(&mut phase.on_start(&mut self.state));
            self.phase = phase;
        }


        //self.phase = next_phase.unwrap();
        
        

        Ok(state_change)
        
        /*let (mut statechange, newphase) = self.phase.handle_action(&mut self.state, action)?;
        if let Some(phase) = newphase {
            self.phase = phase;
            statechange.push(StateChange::PhaseChange(self.phase.phase_type()))
        }
        Ok(statechange)*/
    }
}
