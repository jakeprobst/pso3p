use player::PlayerId;
use phase::phase::PhaseType;
use card::Card;


#[derive(Debug)]
pub enum StateChange {
    NoOp,
    TurnOrder {
        player1_roll: u8,
        player2_roll: u8,
        active_player: PlayerId,
    },

    PhaseChange(PhaseType),

    DrawCard {
        player: PlayerId,
        card: Card,
    },

    Discard(Card),
}
