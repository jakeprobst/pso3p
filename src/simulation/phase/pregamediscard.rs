
use pso3simulation::PSO3State;
use action::{Action, PlayerAction};
use phase::phase::{Phase, PhaseType};
use error::SimulationError;
use statechange::{StateChange};
use player::{Player, PlayerId};
use phase::roll::Roll;
use card::CardInstance;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HandStatus {
    Empty,
    Filled,
    ActedOn,
}

#[derive(Debug, PartialEq)]
struct PlayerData {
    hand_status: HandStatus,
}

impl PlayerData {
    fn new() -> PlayerData {
        PlayerData {
            hand_status: HandStatus::Empty,
        }
    }
}

#[derive(Debug)]
pub struct PreGameDiscard {
    player_status: HashMap<PlayerId, PlayerData>,
}


impl PreGameDiscard {
    pub fn new() -> PreGameDiscard {
        let mut pstatus = HashMap::new();
        pstatus.insert(PlayerId::One, PlayerData::new());
        pstatus.insert(PlayerId::Two, PlayerData::new());
        PreGameDiscard {
            player_status: pstatus,
        }
    }

    pub fn player_hand_status(&mut self, player: PlayerId, status: HandStatus) {
        if let Some(pstatus) = self.player_status.get_mut(&player) {
            pstatus.hand_status = status;
        }
    }

    pub fn both_acted(&self) -> bool {
        for p in [PlayerId::One, PlayerId::Two].iter() {
            if let Some(p1) = self.player_status.get(&p) {
                if p1.hand_status != HandStatus::ActedOn {
                    return false;
                }
            }
        }
        return true;
    }
}


#[derive(Debug)]
pub struct KeepHand {
    player: PlayerId,
}

impl KeepHand {
    pub fn new(p: PlayerId) -> KeepHand {
        KeepHand {
            player: p,
        }
    }
}

impl Action for KeepHand {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        if let Phase::PreGameDiscard(pregamediscard) = phase {
            if let Some(pstatus) = pregamediscard.player_status.get_mut(&self.player) {
                pstatus.hand_status = HandStatus::ActedOn;
            }
        }
        Vec::new()
    }

    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        if let Phase::PreGameDiscard(pregamediscard) = phase {
            if let Some(pstatus) = pregamediscard.player_status.get(&self.player) {
                if pstatus.hand_status == HandStatus::Filled {
                    return true;
                }
            }
        }
        return false;
    }
}


#[derive(Debug)]
pub struct DiscardHand {
    player: PlayerId,
}

impl DiscardHand {
    pub fn new(p: PlayerId) -> DiscardHand {
        DiscardHand {
            player: p,
        }
    }
}

impl Action for DiscardHand {
    fn apply(&self, phase: &mut Phase, state: &mut PSO3State) -> Vec<StateChange> {
        let mut actions = Vec::new();
        if let Phase::PreGameDiscard(pregamediscard) = phase {
            if let Some(pstatus) = pregamediscard.player_status.get_mut(&self.player) {
                pstatus.hand_status = HandStatus::ActedOn;
            }
            let player = state.get_player_mut(self.player);
            for _ in 0..5 {
                let cardinst = player.draw_card();
                actions.push(StateChange::DrawCard {
                    player: player.id,
                    card: cardinst,
                });
            }
        }
        actions
    }

    fn is_valid(&self, phase: &Phase, state: &PSO3State) -> bool {
        if let Phase::PreGameDiscard(pregamediscard) = phase {
            if let Some(pstatus) = pregamediscard.player_status.get(&self.player) {
                if pstatus.hand_status == HandStatus::Filled {
                    return true;
                }
            }
        }
        return false;
    }
}






