



use card::CardId;




#[derive(Debug)]
pub enum PlayerAction {
    RollForFirst,
    DiscardHand,
    KeepHand,
    RollDice,
    SetCard(CardId),

    
    Discard(CardId),
    EndDiscard,
}

#[derive(Debug)]
pub enum Action {
    Player1(PlayerAction),
    Player2(PlayerAction),
}

