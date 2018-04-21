
use std;
use std::sync::atomic::{AtomicU64, Ordering, ATOMIC_U64_INIT};
use serde::ser::Serialize;
use serde::de::Deserialize;

use player::PlayerId;
use card::{Card, CharacterType, CharacterCard, ItemCard, MonsterCard};

pub type ObjectId = u64;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FieldObjectType {
    CharacterHunter,
    CharacterArkz,
    Monster,
    Item,
}

static OBJECT_ID: AtomicU64 =  ATOMIC_U64_INIT;

fn new_object_id() -> ObjectId {
    OBJECT_ID.fetch_add(1, Ordering::SeqCst)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryCharacterFieldObject {
    pub id: ObjectId,
    pub player: PlayerId,
    pub card: CharacterCard,
    pub pos: Position,
}

impl StoryCharacterFieldObject {
    pub fn new(player: PlayerId, card: &CharacterCard, pos: Position) -> StoryCharacterFieldObject {
        StoryCharacterFieldObject {
            id: new_object_id(),
            player: player,
            card: card.clone(),
            pos: pos,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFieldObject {
    pub id: ObjectId,
    pub player: PlayerId,
    card: ItemCard,
    char: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFieldObject {
    pub id: ObjectId,
    pub player: PlayerId,
    card: MonsterCard,
    pos: Position,
}


/*pub enum FieldObject {
    StoryCharacter(StoryCharacterFieldObject),
    Item(ItemFieldObject),
    Monster(MonsterFieldObject),
}

#[derive(Debug, Clone)]
pub struct FieldObjectInstance {
    pub id: ObjectId,
    pub player: PlayerId,
    pub obj: FieldObject,
}

impl FieldObjectInstance {
    pub fn new_sc(player: PlayerId, card: &CharacterCard, pos: Position) -> FieldObjectInstance {
        FieldObjectInstance {
            id: new_object_id(),
            obj: Box::new(FieldObject::StoryCharacter(StoryCharacterFieldObject::new(card, pos)))
        }
    }
}*/

/*#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}*/
