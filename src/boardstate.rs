


use card::*;
use fieldobject::{FieldObjectInstance};
use player::PlayerId;
use field::Field;
use card::Card;

#[derive(Debug)]
pub struct PlayerBoardState {
    pub character: FieldObjectInstance,
    pub in_play: Vec<FieldObjectInstance>,
}



#[derive(Debug)]
pub struct BoardState {
    pub field: Field,
    pub player1: PlayerBoardState,
    pub player2: PlayerBoardState,
}



impl BoardState {
    pub fn new(field: Field, p1_sc: &Card, p2_sc: &Card) -> BoardState {
        BoardState {
            player1: PlayerBoardState {
                character: FieldObjectInstance::new(PlayerId::One, p1_sc, &field.p1start),
                in_play: Vec::new(),
            },
            player2: PlayerBoardState {
                character: FieldObjectInstance::new(PlayerId::Two, p2_sc, &field.p2start),
                in_play: Vec::new(),
            },
            field: field,
        }
    }
}
