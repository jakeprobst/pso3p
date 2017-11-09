



use card::CardId;
use fieldobject::Position;



#[derive(Debug)]
pub enum PlayerAction {
    RollForFirst,
    DiscardHand,
    KeepHand,
    RollDice,
    SetCard(CardId, Position),
    
    
    Discard(CardId),
    EndDiscard,
}

#[derive(Debug)]
pub enum Action {
    Player1(PlayerAction),
    Player2(PlayerAction),
}

