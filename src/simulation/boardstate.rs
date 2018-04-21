


//use card::*;
use fieldobject::{StoryCharacterFieldObject, ItemFieldObject, MonsterFieldObject, Position};
use player::PlayerId;
use field::Field;
use card::{Card, CharacterCard};

use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerBoardState {
    pub character: StoryCharacterFieldObject,
    pub in_play: Vec<MonsterFieldObject>,
    pub equip: Vec<ItemFieldObject>,
}

impl PlayerBoardState {
    fn new(player: PlayerId, sc: &CharacterCard, pos: Position) -> PlayerBoardState {
        PlayerBoardState {
            character: StoryCharacterFieldObject::new(PlayerId::One, sc, pos),
            in_play: Vec::new(),
            equip: Vec::new(),
        }
    }
}


#[derive(Debug)]
pub struct BoardState {
    pub field: Field,
    pub boardstate: HashMap<PlayerId, PlayerBoardState>,
    //pub player1: PlayerBoardState,
    //pub player2: PlayerBoardState,
}



impl BoardState {
    pub fn new(field: Field, p1_sc: &CharacterCard, p2_sc: &CharacterCard) -> BoardState {
        let mut boardstate = HashMap::new();
        boardstate.insert(PlayerId::One, PlayerBoardState::new(PlayerId::One, p1_sc, field.p1start));
        boardstate.insert(PlayerId::Two, PlayerBoardState::new(PlayerId::Two, p2_sc, field.p2start));
        BoardState {
            boardstate: boardstate,
            field: field,
        }
    }

    //pub fn get_object(&self, object: ObjectId) -> 
       
}
