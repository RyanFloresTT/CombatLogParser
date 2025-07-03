use crate::models::LogEvent;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DUNGEON_RE: Regex = Regex::new(r#"CHALLENGE_MODE_START,"([^"]+)""#).unwrap();
    static ref COMBATANT_RE: Regex = Regex::new(r#"COMBATANT_INFO,(Player-[^,]+)"#).unwrap();
    static ref GUID_NAME_RE: Regex = Regex::new(r#"(Player-[^,]+),"([^"]+)""#).unwrap();
    static ref DAMAGE_RE: Regex = Regex::new(r#"SPELL_DAMAGE,.*,(Player-[^,]+),[^,]*,[^,]*,[^,]*,(\d+),"#).unwrap();
    static ref DUNGEON_END_RE: Regex = Regex::new(r#"CHALLENGE_MODE_END"#).unwrap();
}

/* TODO:
    1. Keep track of current dungeon
    2. Parse the first 5 player ID's and match them to their names
    3. Total the damage and healing for each player for each dungeon
    4. Stop reading lines when a dungeon has ended, repeat once a new dungeon is found
 */
pub fn parse_line(line: &str) -> LogEvent {
    if let Some(cap) = DUNGEON_RE.captures(line) {
        return LogEvent::DungeonStart {
            name: cap[1].to_string(),
            timestamp: extract_timestamp(line),
        };
    }

    if DUNGEON_END_RE.is_match(line) {
        return LogEvent::DungeonEnd {
            timestamp: extract_timestamp(line),
        };
    }

    if let Some(cap) = COMBATANT_RE.captures(line) {
        return LogEvent::CombatantInfo {
            guid: cap[1].to_string(),
        };
    }

    if let Some(cap) = GUID_NAME_RE.captures(line) {
        return LogEvent::GuidToName {
            guid: cap[1].to_string(),
            name: cap[2].to_string(),
        };
    }

    if let Some(cap) = DAMAGE_RE.captures(line) {
        return LogEvent::SpellDamage {
            guid: cap[1].to_string(),
            amount: cap[2].parse().unwrap_or(0),
        };
    }

    LogEvent::Unknown
}

fn extract_timestamp(line: &str) -> String {
    line.split_whitespace().next().unwrap_or("???").to_string()
}
