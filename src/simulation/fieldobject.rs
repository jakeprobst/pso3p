

use std::sync::atomic::{AtomicU64, Ordering, ATOMIC_U64_INIT};


use player::PlayerId;
use card::{Card, CharacterType};

pub type ObjectId = u64;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldObjectType {
    CharacterHunter,
    CharacterArkz,
    Monster,
    Item,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldObjectInstance {
    pub id: ObjectId,
    #[serde(rename = "type")]
    pub ftype: FieldObjectType,
    pub player: PlayerId,
    pub card: Card,
    pub pos: Position,
    pub hp: u8,
    //pub ap,
    //pub tp,
    //pub mv,
}

static OBJECT_ID: AtomicU64 =  ATOMIC_U64_INIT;

fn new_object_id() -> ObjectId {
    OBJECT_ID.fetch_add(1, Ordering::SeqCst)
}

impl FieldObjectInstance {
    pub fn new(player: PlayerId, card: &Card, pos: &Position) -> FieldObjectInstance {
        let ftype = match *card {
            Card::Character(ref c) => match c.ctype {
                CharacterType::Hunter => FieldObjectType::CharacterHunter,
                CharacterType::Arkz => FieldObjectType::CharacterArkz,
            }
            Card::Monster(_) => FieldObjectType::Monster,
            Card::Item(_) => FieldObjectType::Item,
            _ => panic!("trying to make field object instance of invalid type {:?}", card),
        };
        let hp = match *card {
            Card::Character(ref c) => c.hp as u8,
            Card::Monster(ref m) => m.hp,
            Card::Item(ref i) => i.hp,
            _ => panic!("trying to make field object instance of invalid type {:?}", card),
        };
        FieldObjectInstance {
            id: new_object_id(),
            ftype: ftype,
            player: player,
            card: card.clone(),
            hp: hp,
            pos: pos.clone(),
        }
    }
}
