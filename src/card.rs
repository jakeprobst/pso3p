
use std::sync::atomic::{AtomicU64, Ordering, ATOMIC_U64_INIT};

use player::PlayerId;

pub type CardId = u64;


// TODO: bother with boss only colors?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionLink {
    Red,
    Yellow,
    Blue,
    Green,
    Purple,
    Orange,
    Pollux,
    Castor,
    Leukon,
    Amplum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MonsterType {
    Native,
    ABeast,
    Machine,
    Dark,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemType {
    Sword,
    Gun,
    Cane,
    Shield,
    Mag,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CharacterType {
    Hunter,
    Arkz,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TargetType {
    Single,
    Multiple,
    Team,
    Ally,
    #[serde(rename = "Self")]
    TSelf,
    Everyone,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ability {
    ABeastAPBoost,
    ABeastAPCount,
    ABeastAttack,
    ABeastHalfguard,
    ABeastInfluence,
    ABeastShield,
    ABeastSlayer,
    ACTrash,
    AHSwap,
    AHSwapTurn,
    APAssist,
    APBoost,
    APGrowth,
    APLoss,
    APRise,
    APSilence,
    ASReturn,
    ASTrash,
    ASVanish,
    ATKDiceBoost,
    ATSwap,
    ATSwapPerm,
    ATSwapTurn,
    AbilityBoost,
    AbilityRise,
    AbilityTrap,
    Acid,
    ActionCostDecrease,
    ActionCostIncrease,
    ActionDisrupter,
    ActionPointSwap,
    ActionUp,
    Actionx2,
    Aerial,
    AerialAttack,
    AerialAssassin,
    AerialStatus,
    AerialTeam,
    AllyBonus,
    AllyGuard,
    AllyRecovery,
    AllySCAPBoost,
    AntiAbnormality,
    Artifice,
    AshesofDeath,
    Assault,
    AssistBanned,
    AssistBlock,
    AssistChange,
    AssistReverse,
    AttackBlock,
    BattleRecovery,
    BattleRoyale,
    Berserk,
    BigHand,
    BigSwing,
    BloodSuck,
    Bomb,
    BonusAPRise,
    BonusTPRise,
    Breed,
    Brown,
    CaneAttack,
    CaneDemolisher,
    CaneShield,
    CaneTPBoost,
    CaneTPCount,
    CardBack,
    CaneHalfguard,
    CarnageInclined,
    CarnageSpeedup,
    CarnageUninclined,
    Clone,
    Combo,
    ContactHalfguard,
    Copy,
    Counter,
    Curse,
    DEFCost1Disable,
    DEFCost2Disable,
    DEFCost3Disable,
    DamageAbsorb,
    DarkAPBoost,
    DarkAPCount,
    DarkAttack,
    DarkHalfguard,
    DarkShield,
    DarkSlayer,
    Dash,
    DeathCompanion,
    Decoy,
    DecoyRecovery,
    DefenderAPGrowth,
    DefenseDisable,
    DevilsWhim,
    Dice1,
    Dice12,
    Dice5,
    Dice6,
    DiceBonus,
    DiceMinus,
    DoubleStrike,
    DoubleorNothing,
    Drain,
    DrawLess,
    Drop,
    DuoTechStrike,
    EXPBonus,
    EXPDecoy,
    EXPLoss,
    EarthBind,
    EffectReducer,
    EffectTimex2,
    Elude,
    EnemyAHSwap,
    EnemyAHSwapPerm,
    EnemyAPBoost,
    EnemyATSwap,
    EnemyBonus,
    EqualDamager,
    EquipperEXPBoost,
    EquipperHaste,
    EquipperHeal,
    Exhaust,
    Explosion,
    FCAPBoost,
    FCATSwapPerm,
    FCBanned,
    FCBonus,
    FCTrash,
    Feeble,
    FetchDiscarded,
    Filial,
    FixedAP,
    FixedDamage,
    FixedHP,
    FixedRange,
    Flee,
    Focus,
    FocusedAssault,
    FreeManeuver,
    FreeSummoning,
    Freeze,
    FrontalRange,
    FrozenDeath,
    FrozenTarget,
    FullAP,
    FullForce,
    FullHeal,
    FullAPAssist,
    FullTPAssist,
    GhostAttack,
    GoldRush,
    Group,
    Grudge,
    Guard,
    GuardCreature,
    GuardsDemolisher,
    GunAPBoost,
    GunAPCount,
    GunAttack,
    GunDemolisher,
    GunHalfguard,
    GunShield,
    Guom,
    HP2Defense,
    HPAssist,
    HPChange,
    HPHalver,
    HPPower,
    HPRegain,
    HandDisrupter,
    HandFlip,
    Haste,
    Heal,
    HeavyItem,
    HeavyPierce,
    HeavyRampage,
    HeavySteps,
    HighcostAPBoost,
    HighcostDamager,
    HighcostGuard,
    HighcostHalfguard,
    HighcostSlayer,
    Hold,
    Immobile,
    ImmobileHold,
    Immortal,
    ImpactBlock,
    ImpactHalfguard,
    Inherit,
    Insanity,
    InstantDeath,
    Interest,
    Leader,
    Legacy,
    Leilla,
    LightItem,
    LimitedAction,
    LinkAPBoost,
    LinkAPSpeedup,
    LinkEXPDecoy,
    LinkHeal,
    LinkInherit,
    LinkReturn,
    LinkSnatch,
    LowcostAPBoost,
    LowcostDEFDisable,
    LowcostDamager,
    LowcostGuard,
    LowcostHalfguard,
    LowcostSlayer,
    MV1,
    MV9,
    MachineAPBoost,
    MachineAPCount,
    MachineAttack,
    MachineHalfguard,
    MachineInfluence,
    MachineShield,
    MachineSlayer,
    Mag,
    MajorDamager,
    MajorDeath,
    MajorHalfguard,
    MajorPierce,
    MajorRampage,
    MajorSlayer,
    MeteorShower,
    MinorDeath,
    MinorHalfguard,
    MinorSlayer,
    MentalFocus,
    MultipleStrike,
    MutualDeath,
    MyllaYoulla,
    NativeAPBoost,
    NativeAPCount,
    NativeAttack,
    NativeHalfguard,
    NativeInfluence,
    NativeShield,
    NativeSlayer,
    Offering,
    Paralysis,
    ParalyzedDeath,
    Parry,
    PeriodicField,
    PhotonBlast,
    PhysicalAttackBlock,
    Pierce,
    PierceBlock,
    PierceReflect,
    Protector,
    Quake,
    Rampage,
    RampageAPLoss,
    RampageReflect,
    RandomAP,
    RandomAssist,
    RandomTP,
    ReactionSlowdown,
    Rebirth,
    Reborn,
    Remedy,
    Requiem,
    Return,
    ReturnHome,
    Revenge,
    ReverseTech,
    Revolt,
    Rigid,
    Ripple,
    RussianRoulette,
    SCRecovery,
    SCSlayer,
    SameAPBlock,
    SameCardBanned,
    ShadowBind,
    ShieldWeapon,
    ShockWave,
    Shuffle,
    SkipAct,
    SkipDraw,
    SkipMove,
    SkipSet,
    SkipTurn,
    Snatch,
    SpecialAttackBlock,
    SpecialReflect,
    Stall,
    SteadyDamage,
    SteadyDamageMod,
    Suicide,
    Summon,
    Survival,
    SurvivalDecoys,
    SurvivorAPBoost,
    SurvivorRecovery,
    SwordAPBoost,
    SwordAPCount,
    SwordAttack,
    SwordBonus,
    SwordDemolisher,
    SwordHalfguard,
    SwordShield,
    TPAssist,
    TPBoost,
    TPGrowth,
    TPLoss,
    TPPower,
    TPSilence,
    Tech,
    TechEnable,
    TechHalfguard,
    TechReflect,
    Techx2,
    TimeBomb,
    TimedAPSacrifice,
    TimedEXPSacrifice,
    TimedFrozenDeath,
    TimedHPSacrifice,
    TimedParalyzed,
    TimedPierce,
    TimedTPSacrifice,
    Traitor,
    Trash1,
    TripleStrike,
    UltimateAPGrowth,
    Unfilial,
    UnlimitedSummoning,
    ValueRise,
    Vengeance,
    Warp,
    Waste,
    WeakHitBlock,
    WeakSpot,
}


// TODO: how the fuck should I do this?
//#[derive(Serialize, Deserialize, Debug, Clone)]
pub type Range = Vec<String>;
/*pub struct Range {
    
}*/


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub ctype: CharacterType,
    pub class: CharacterClass,
    pub hp: i8,
    pub ap: i8,
    pub tp: i8,
    pub mv: i8,
    pub target: TargetType,
    pub range: Range,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemCard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub ctype: ItemType,
    pub cost: u8,
    pub hp: u8,
    pub ap: u8,
    pub tp: u8,
    pub mv: u8,
    pub target: TargetType,
    pub range: Range,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonsterCard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub ctype: MonsterType,
    pub cost: u8,
    pub hp: u8,
    pub ap: u8,
    pub tp: u8,
    pub mv: u8,
    pub target: TargetType,
    pub range: Range,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    Attack,
    Defense,
    PhotonBlast
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionCard {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub ctype: ActionType,
    pub cost: u8,
    pub hp: u8,
    pub ap: u8,
    pub tp: u8,
    pub mv: u8, // TODO: I think I can get rid of this
    pub target: TargetType,
    pub range: Range,
    pub leftlink: Vec<ActionLink>,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistCard {
    pub id: u32,
    pub name: String,
    pub cost: u8,
    pub time: u8,
    pub target: TargetType,
    pub ability: Vec<Ability>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BossCard {
    pub id: u32,
    pub name: String,
    pub hp: i8,
    pub ap: i8,
    pub tp: i8,
    pub mv: i8,
    pub target: TargetType,
    pub range: Range,
    pub toplink: Vec<ActionLink>,
    pub rightlink: Vec<ActionLink>,
    pub ability: Vec<Ability>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Card {
    Character(CharacterCard),
    Item(ItemCard),
    Monster(MonsterCard),
    Action(ActionCard),
    Assist(AssistCard),
    Boss(BossCard),
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardInstance {
    id: CardId,
    pub card: Card,
}


static CARD_ID: AtomicU64 =  ATOMIC_U64_INIT;

fn new_card_id() -> CardId {
    CARD_ID.fetch_add(1, Ordering::SeqCst)
}



impl CardInstance {
    pub fn new(card: Card) -> CardInstance {
        CardInstance {
            id: new_card_id(),
            card: card,
        }
    }

    pub fn id(&self) -> CardId {
        self.id
    }
}
