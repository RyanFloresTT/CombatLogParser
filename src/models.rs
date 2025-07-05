use std::collections::HashMap;

#[derive(Debug)]
pub enum LogEvent {
    DungeonStart { name: String, timestamp: String },
    DungeonEnd { timestamp: String },
    CombatantInfo { guid: String },
    GuidToName { guid: String, name: String },
    SpellDamage { guid: String, amount: u64 },
    Unknown,
}

#[derive(Debug)]
pub struct DungeonRun {
    pub name: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub players: HashMap<String, String>,     // GUID -> name
    pub damage_by_guid: HashMap<String, u64>, // GUID -> total damage
}

impl DungeonRun {
    pub fn new(name: String, start_time: String) -> Self {
        Self {
            name,
            start_time,
            end_time: None,
            players: HashMap::new(),
            damage_by_guid: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, guid: String, name: Option<String>) {
        if !(name.is_none() && self.players.contains_key(&guid)) {
            self.players.insert(guid, name.unwrap_or("Unknown".into()));
        }
    }

    pub fn set_end_time(&mut self, ts: String) {
        self.end_time = Some(ts);
    }

    pub fn apply_damage(&mut self, guid: String, amount: u64) {
        *self.damage_by_guid.entry(guid).or_insert(0) += amount;
    }
}