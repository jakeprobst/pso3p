



use card::Card;
use player::PlayerId;




#[derive(Debug)]
pub enum Action {
    RollForFirstPlayer,
    DiscardHand(PlayerId),
    KeepHand(PlayerId),

    
    Discard(Card),
    EndDiscard,
}



