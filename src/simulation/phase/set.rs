use player::{Player, PlayerId};
use boardstate::PlayerBoardState;
use phase::phase::{Phase, PhaseType};
use action::{Action, PlayerAction};
use error::SimulationError;
use statechange::StateChange;
use pso3simulation::PSO3State;
use card::{Card, CardId};
use fieldobject::{Position, ObjectId};

#[derive(Debug)]
pub struct Set;


impl Set {
    pub fn new() -> Set {
        Set {
        }
    }
}



#[derive(Debug)]
struct SetMonsterCard {
    player: PlayerId,
    card: CardId,
    pos: Position,
}


impl SetMonsterCard {
    fn new(player: PlayerId, card: CardId, pos: Position) -> SetMonsterCard {
        SetMonsterCard {
            player: player,
            card: card,
            pos: pos,
        }
    }
}


impl Action for SetMonsterCard {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        let player = state.get_active_player_mut();
        let card = player.pop_card(self.card);
        
        
        
        Vec::new()
    }

    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        if self.player != state.active_player {
            return false;
        }

        let player = state.get_active_player();
        let card = player.get_card(self.card);

        if let Some(c) = card {
            if let Card::Monster(ref monster) = c.card {
                // TODO: costs, placement, etc checks
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
        

        return true;
    }
}


#[derive(Debug)]
struct SetItemCard {
    player: PlayerId,
    card: CardId,
    target: ObjectId,
}


impl SetItemCard {
    fn new(player: PlayerId, card: CardId, target: ObjectId) -> SetItemCard {
        SetItemCard {
            player: player,
            card: card,
            target: target,
        }
    }
}

impl Action for SetItemCard {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        Vec::new()
    }
    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        if self.player != state.active_player {
            return false;
        }

        let player = state.get_active_player();
        let cinst = match player.get_card(self.card) {
            Some(c) => c,
            None => return false,
        };

        let card = match cinst.card.as_item() {
            Some(i) => i,
            None => return false,
        };
        
        //let target = player.get_card(self.card);
        //let target = state.boardstate.get_object(self.target);

        
        //if self.player != 

        /*if let Some(c) = card {
            if let Card::Item(ref item) = c.card {
                if player.atk >= item.cost {
                    return false;
                }
                

                
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }*/
        


        false
    }
}


struct SetAssistCard {
}


/*
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
        _ => Err(SimulationError::InvalidAction(PhaseType::Set, action))
    }
}

impl Phase for Set {
    fn handle_action(&mut self, state: &mut PSO3State, action: Action)
                     -> Result<(Vec<StateChange>, Option<Box<Phase + Send>>), SimulationError> {
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

*/
