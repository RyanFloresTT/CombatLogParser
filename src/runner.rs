use crate::models::DungeonRun;
use crate::parser::parse_line;
use crate::models::LogEvent;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(path: &str) {
    let file = File::open(path).expect("Failed to open log file");
    let reader = BufReader::new(file);

    let mut current_dungeon: Option<DungeonRun> = None;
    let mut all_dungeons = vec![];

    for line in reader.lines().flatten() {
        let event = parse_line(&line);

        match event {
            LogEvent::DungeonStart { name, timestamp } => {
                if let Some(prev) = current_dungeon.take() {
                    all_dungeons.push(prev);
                }
                current_dungeon = Some(DungeonRun::new(name, timestamp));
            }

            LogEvent::DungeonEnd { timestamp } => {
                if let Some(d) = current_dungeon.as_mut() {
                    d.set_end_time(timestamp);
                }
            }

            LogEvent::CombatantInfo { guid } => {
                if let Some(d) = current_dungeon.as_mut() {
                    d.add_player(guid, None);
                }
            }

            LogEvent::GuidToName { guid, name } => {
                if let Some(d) = current_dungeon.as_mut() {
                    d.add_player(guid, Some(name));
                }
            }

            LogEvent::SpellDamage { guid, amount } => {
                if let Some(d) = current_dungeon.as_mut() {
                    d.apply_damage(guid, amount);
                }
            }

            LogEvent::Unknown => {}
        }
    }

    if let Some(d) = current_dungeon {
        all_dungeons.push(d);
    }

    for dungeon in all_dungeons {
        println!("Dungeon: {} [{} - {:?}]", dungeon.name, dungeon.start_time, dungeon.end_time);
        for (guid, name) in &dungeon.players {
            println!(" - {}: {} did {} damage.", guid, name, dungeon.damage_by_guid.get(guid).unwrap_or(&0));
        }
    }
}
