



use card::Card;
use player::PlayerId;




pub enum Action {
    RollForFirstPlayer,
    DiscardHand(PlayerId),
    KeepHand(PlayerId),

    
    Discard(Card),
    EndDiscard,
}



