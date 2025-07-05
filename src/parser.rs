use crate::models::LogEvent;
use crate::util::extract_timestamp;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DUNGEON_RE: Regex = Regex::new(r#"CHALLENGE_MODE_START,"([^"]+)""#).unwrap();
    static ref COMBATANT_RE: Regex = Regex::new(r#"COMBATANT_INFO,(Player-[^,]+)"#).unwrap();
    static ref GUID_NAME_RE: Regex = Regex::new(r#"(Player-[^,]+),"([^"]+)""#).unwrap();
    static ref DAMAGE_RE: Regex =
        Regex::new(r#"SPELL_DAMAGE,.*,(Player-[^,]+),[^,]*,[^,]*,[^,]*,(\d+),"#).unwrap();
    static ref DUNGEON_END_RE: Regex = Regex::new(r#"CHALLENGE_MODE_END"#).unwrap();
}

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
            name: cap[2]
                .trim_matches('"')
                .split('-')
                .next()
                .unwrap_or(cap[2].trim_matches('"'))
                .to_string(),
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
