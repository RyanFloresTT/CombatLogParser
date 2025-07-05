use crate::models::LogEvent;
use regex::Regex;

/* TODO:
   1. Keep track of current dungeon
   2. Parse the first 5 player ID's and match them to their names
   3. Total the damage and healing for each player for each dungeon
   4. Stop reading lines when a dungeon has ended, repeat once a new dungeon is found
*/
pub fn parse_line(line: &str) -> LogEvent {
    let dungeon_re = Regex::new(r#"CHALLENGE_MODE_START,"([^"]+)""#).unwrap();
    let combatant_re = Regex::new(r#"COMBATANT_INFO,(Player-[^,]+)"#).unwrap();
    let guid_name_re = Regex::new(r#"(Player-[^,]+),"([^"]+)""#).unwrap();
    let damage_re =
        Regex::new(r#"SPELL_DAMAGE,.*,(Player-[^,]+),[^,]*,[^,]*,[^,]*,(\d+),"#).unwrap();
    let dungeon_end_re = Regex::new(r#"CHALLENGE_MODE_END"#).unwrap();

    if let Some(cap) = dungeon_re.captures(line) {
        return LogEvent::DungeonStart {
            name: cap[1].to_string(),
            timestamp: extract_timestamp(line),
        };
    }

    if dungeon_end_re.is_match(line) {
        return LogEvent::DungeonEnd {
            timestamp: extract_timestamp(line),
        };
    }

    if let Some(cap) = combatant_re.captures(line) {
        return LogEvent::CombatantInfo {
            guid: cap[1].to_string(),
        };
    }

    if let Some(cap) = guid_name_re.captures(line) {
        return LogEvent::GuidToName {
            guid: cap[1].to_string(),
            name: cap[2].to_string(),
        };
    }

    if let Some(cap) = damage_re.captures(line) {
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
