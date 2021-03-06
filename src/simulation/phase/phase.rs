

use pso3simulation::PSO3State;
use action::Action;
use statechange::StateChange;
use error::SimulationError;
use player::PlayerId;
use phase::*;


//#[derive(Debug, Clone)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhaseType {
    GameStart,
    PreGameDiscard,
    Roll,
    Set
}

//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Debug)]
pub enum Phase {
    Init,
    GameStart(gamestart::GameStart),
    PreGameDiscard(pregamediscard::PreGameDiscard),
    Roll(roll::Roll),
    Set(set::Set),
    /*Move,
    Attack,
    Defend,
    Draw,*/
}


//pub struct PhaseType<'a>(&'a str);


/*pub fn is_active_player(action: &Action, playerid: PlayerId) -> bool {
    match *action {
        Action::Player1(_) => PlayerId::One == playerid,
        Action::Player2(_) => PlayerId::Two == playerid,
    }
}*/

//pub fn get_player_data()


#[derive(Debug)]
pub struct Start {
}

impl Action for Start {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        Vec::new()
    }

    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        true
    }
}


impl Phase {
    fn pregamediscard_start(&self, pregamediscard: &mut pregamediscard::PreGameDiscard, state: &mut PSO3State) -> Vec<StateChange> {
        let mut actions = Vec::new();
        actions.push(StateChange::PhaseChange(PhaseType::PreGameDiscard));
        //for player in [state.get_player(PlayerId::One), state.get_player(PlayerId::Two)].iter_mut() {
        for player in [&mut state.player1, &mut state.player2].iter_mut() {
            for _ in 0..5 {
                let cardinst = player.draw_card();
                actions.push(StateChange::DrawCard {
                    player: player.id,
                    card: cardinst,
                });
            }
            pregamediscard.player_hand_status(player.id, pregamediscard::HandStatus::Filled);
        };
        actions
    }
    
    pub fn on_start(&mut self, state: &mut PSO3State) -> Vec<StateChange> {
        match self {
            Phase::GameStart(gamestart) => {
                //self.gamestart_start(state)
                gamestart.on_start(state)
            },
            Phase::PreGameDiscard(ref mut pregamediscard) => {
                self.pregamediscard_start(pregamediscard, state)
            },
            Phase::Roll(roll) => {
                vec![roll.roll_atk_def(state)]
            },
            _ => Vec::new()
        }
    }
    pub fn next_phase(&self) -> Option<Phase> {
        match self {
            Phase::Init => {
                return Some(Phase::GameStart(gamestart::GameStart::new()));
            }
            Phase::GameStart(gamestart) => {
                if gamestart.both_rolled() {
                    return Some(Phase::PreGameDiscard(pregamediscard::PreGameDiscard::new()));
                }
            },
            Phase::PreGameDiscard(pregamediscard) => {
                if pregamediscard.both_acted() {
                    return Some(Phase::Roll(roll::Roll::new()));
                }
            },
            Phase::Roll(roll) => {
                return Some(Phase::Set(set::Set::new()));
            }
            
            _ => {},
        }
        None
    }
    //fn data(&self) -> Self::Data;
}



/*pub trait Phase {
    //fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     //-> Result<(Vec<StateChange>, Option<Box<Phase + Send>>), SimulationError>;
    fn phase_type(&self) -> PhaseType;
}*/

