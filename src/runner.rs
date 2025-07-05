use crate::models::DungeonRun;
use crate::models::LogEvent;
use crate::parser::parse_line;
use colored::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(path: &str) {
    let file = File::open(path).expect("Failed to open log file");
    let reader = BufReader::new(file);

    let mut current_dungeon: Option<DungeonRun> = None;
    let mut all_dungeons = vec![];
    let mut current_dungeon_players = HashSet::new();

    for line in reader.lines().flatten() {
        let event = parse_line(&line);

        match event {

            LogEvent::DungeonStart { name, timestamp } => {
                if let Some(prev) = current_dungeon.take() {
                    all_dungeons.push(prev);
                }
                current_dungeon = Some(DungeonRun::new(name, timestamp));
                current_dungeon_players.clear(); // Reset for new a dungeon
            }

            LogEvent::CombatantInfo { guid } => {
                if let Some(d) = current_dungeon.as_mut() {
                    current_dungeon_players.insert(guid.clone());
                    d.add_player(guid, None);
                }
            }

            LogEvent::GuidToName { guid, name } => {
                if let Some(d) = current_dungeon.as_mut() {
                    if current_dungeon_players.contains(&guid) {
                        d.add_player(guid, Some(name));
                    }
                }
            }

            LogEvent::SpellDamage { guid, amount } => {
                if let Some(d) = current_dungeon.as_mut() {
                    if current_dungeon_players.contains(&guid) {
                        d.apply_damage(guid, amount);
                    }
                }
            }

            LogEvent::DungeonEnd { timestamp } => {
                if let Some(d) = current_dungeon.as_mut() {
                    d.set_end_time(timestamp);
                }
            }

            LogEvent::Unknown => {}
        }
    }

    if let Some(d) = current_dungeon {
        all_dungeons.push(d);
    }

    // Final summary
    println!("\n{}", "Final Dungeon Reports:".bold());
    for (i, dungeon) in all_dungeons.iter().enumerate() {
        println!(
            "\n{} {} [{} - {}]",
            format!("Dungeon #{}:", i + 1).bold(),
            dungeon.name.green(),
            dungeon.start_time.time().format("%H:%M").to_string(),
            dungeon.end_time.unwrap().time().format("%H:%M").to_string()
        );

        // Only show the actual dungeon participants
        let mut players: Vec<_> = dungeon
            .players
            .iter()
            .filter(|(guid, _)| current_dungeon_players.contains(*guid))
            .collect();
        players.sort_by(|(_, name1), (_, name2)| name1.cmp(name2));

        println!("Players:");
        for (guid, name) in players {
            let damage = dungeon.damage_by_guid.get(guid).unwrap_or(&0);
            println!("  {}: {} damage", name, damage);
        }
    }
}
