use player::PlayerId;
use phase::phase::PhaseType;
use card::{CardInstance, CardId};
use fieldobject::{FieldObjectInstance, Position};

#[derive(Debug)]
pub enum StateChange {
    NoOp,
    TurnOrderRolls {
        player1_roll: u8,
        player2_roll: u8,
    },
    TurnChange {
        player: PlayerId,
    },
    AtkDefDiceRoll {
        player: PlayerId,
        atk: u8,
        def: u8,
    },
    PhaseChange(PhaseType),

    DrawCard {
        player: PlayerId,
        card: CardInstance,
    },

    SetCard {
        player: PlayerId,
        card: FieldObjectInstance,
        pos: Position,
    },

    /*DiscardHand {
        player: PlayerId,
    },*/

    DiscardCard {
        player: PlayerId,
        card: CardId,
    }
}
