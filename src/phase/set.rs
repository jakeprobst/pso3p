



use player::Player;
use boardstate::PlayerBoardState;
use phase::phase::{Phase, PhaseType};
use action::{Action, PlayerAction};
use error::SimulationError;
use statechange::StateChange;
use pso3simulation::PSO3State;
use card::{Card, CardId};
use fieldobject::{Position, FieldObjectInstance};

pub struct Set;


impl Set {
    pub fn new() -> Set {
        Set {
        }
    }
}


fn set_card(card_id: CardId, pos: Position, state: &mut PlayerBoardState, player: &mut Player)
            -> Result<Vec<StateChange>, SimulationError> {

    if let Some(cardinst) = player.hand_card_by_id(card_id) {
        let mut actions = Vec::new();

        let fobj = FieldObjectInstance::new(player.id, &cardinst.card, &pos);
        state.in_play.push(fobj.clone());
        actions.push(StateChange::SetCard {
            player: player.id,
            card: fobj,
            pos: pos,
        });

        actions.push(StateChange::DiscardCard {
            player: player.id,
            card: card_id,
        });

        player.hand_discard_card(cardinst);
        Ok(actions)
    }
    
    else {
        Err(SimulationError::InvalidAction(PhaseType::Set, PlayerAction::SetCard(card_id, pos)))
    }
}

fn handle_player_action(action: PlayerAction, board: &mut PlayerBoardState, player: &mut Player)
                        -> Result<Vec<StateChange>, SimulationError> {
    match action {
        PlayerAction::SetCard(cardid, pos) => set_card(cardid, pos, board, player),
        _ => Err(SimulationError::InvalidAction(PhaseType::Roll, action))
    }
}

impl Phase for Set {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase>>), SimulationError> {
        let actions = match action {
            Action::Player1(act) => handle_player_action(act, &mut state.boardstate.player1, &mut state.player1)?,
            Action::Player2(act) => handle_player_action(act, &mut state.boardstate.player2, &mut state.player2)?,
        };

        Ok((actions, None))
    }

    fn phase_type(&self) -> PhaseType {
        PhaseType::Set
    }
    
}
