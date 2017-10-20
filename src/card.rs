




// TODO: bother with boss only colors?
pub enum ActionLink {
    Red,
    Yellow,
    Blue,
    Green,
    Purple,
    Orange,
}

pub enum MonsterType {
    Native,
    ABeast,
    Machine,
    Dark,
}

pub enum ItemType {
    Sword,
    Gun,
    Cane,
    Shield,
    Mag,
}


pub enum CharacterType {
    Hunter,
    Arkz,
}

pub enum CharacterClass {
    HUmar,
    HUnewearl,
    HUcast,
    HUcaseal,
    RAmar,
    RAmarl,
    RAcast,
    RAcaseal,
    FOmar,
    FOmarl,
    FOnewm,
    FOnewearl,
}

pub enum TargetType {
    Single,
    Multiple,
    Team,
    TSelf,
}

pub enum Ability {
    SwordBonus,
}


// TODO: how the fuck should I do this?
pub struct Range {
}


pub struct CharacterCard {
    name: String,
    ctype: CharacterType,
    class: CharacterClass,
    hp: u8,
    ap: u8,
    tp: u8,
    mv: u8,
    target: TargetType,
    range: Range,
    toplink: Vec<ActionLink>,
    rightlink: Vec<ActionLink>,
    abilities: Vec<Ability>,
    
}

pub struct ItemCard {
    name: String,
    ctype: ItemType,
    cost: u8,
    hp: u8,
    ap: u8,
    tp: u8,
    mv: u8,
    target: TargetType,
    range: Range,
    toplink: Vec<ActionLink>,
    rightlink: Vec<ActionLink>,
    abilities: Vec<Ability>,
}

pub struct MonsterCard {
    name: String,
    ctype: MonsterType,
    cost: u8,
    hp: u8,
    ap: u8,
    tp: u8,
    mv: u8,
    target: TargetType,
    range: Range,
    toplink: Vec<ActionLink>,
    rightlink: Vec<ActionLink>,
    abilities: Vec<Ability>,
}


pub enum ActionType {
    Attack,
    Defense,
}

pub struct ActionCard {
    name: String,
    ctype: ActionType,
    cost: u8,
    hp: u8,
    ap: u8,
    tp: u8,
    mv: u8, // TODO: I think I can get rid of this
    target: TargetType,
    range: Range,
    leftlink: Vec<ActionLink>,
    toplink: Vec<ActionLink>,
    rightlink: Vec<ActionLink>,
    abilities: Vec<Ability>,
}

pub struct AssistCard {
    name: String,
    ctype: ActionType,
    cost: u8,
    time: u8,
    target: TargetType,
    abilities: Vec<Ability>,
}











pub enum Card {
    Character(CharacterCard),
    Item(ItemCard),
    Monster(MonsterCard),
    Action(ActionCard),
    Assist(AssistCard),
}






