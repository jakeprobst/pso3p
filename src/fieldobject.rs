




use player::PlayerId;
use card::Card;


/*pub struct FieldObject {
    player: PlayerId,
    name: String,
    cost: u8,
    hp: i8,
    ap: i8,
    tp: i8,
    mv: i8,
    
    pub target: TargetType,
    pub range: Range,
    pub leftlink: Vec<ActionLink>,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
    



    
}*/

pub enum FieldObjectError {
    InvalidCard(Card),
}



pub struct FieldCharacter {
    
}


pub struct FieldMonster {
    
}


pub struct FieldItem {
    
}



pub enum FieldObject {
    Character {
    },
    Monster {
    },
    Item {
    },
}


pub struct FieldObjectInstance {
    pub id: u32,
    pub object: FieldObject,
}




impl FieldObject {
    //pub fn new(player: PlayerId, card: &Card) -> FieldObject {
    //}
}
