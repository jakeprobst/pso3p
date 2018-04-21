use player::PlayerId;
use phase::phase::PhaseType;
use card::{CardInstance, CardId};
use fieldobject::{StoryCharacterFieldObject, ItemFieldObject, MonsterFieldObject, Position};



/*pub enum StateChange {
    Player1(StateChangeAction),
    Player2(StateChangeAction),
    Both(StateChangeAction),
}*/

//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Debug, Clone, Serialize, Deserialize)]
//#[derive(Debug)]
pub enum StateChange {
    NoOp,
    DebugMsg(String),
    SetField { // TODO
        width: u8,
        height: u8,
    },
    SetPlayer(PlayerId),
    SetCharacter(StoryCharacterFieldObject),
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

    SetItemCard(ItemFieldObject),

    SetMonsterCard(MonsterFieldObject),

    /*DiscardHand {
        player: PlayerId,
    },*/

    DiscardCard {
        player: PlayerId,
        card: CardId,
    }
}
