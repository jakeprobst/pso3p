

use player::PlayerId;
use phase::phase::PhaseType;
use card::Card;


pub struct TurnOrder {
    pub player1_roll: u8,
    pub player2_roll: u8,
    pub active_player: PlayerId,
}



pub enum StateChange {
    NoOp,
    TurnOrder(TurnOrder),

    PhaseChange(PhaseType),


    Discard(Card),
}
