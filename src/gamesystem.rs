use super::{Skill, Skills};

pub fn attr_bonus(value: i32) -> i32 {
    (value-10)/2 // Standard stat block modifier values from https://roll20.net/compendium/dnd5e/Ability%20Scaores#content
}

pub fn player_hp_per_level(integrity: i32) -> i32 {
    10 + attr_bonus(integrity)
}

pub fn player_hp_at_level(integrity: i32, level:i32) -> i32 {
    player_hp_per_level(integrity) * level
}

pub fn npc_hp(integrity: i32, level: i32) -> i32 {
    let mut total = 1;
    for _i in 0..level {
        total += i32::max(1, 8 + attr_bonus(integrity));
    }
    total
}

pub fn energy_per_level(compute: i32) -> i32 {
    i32::max(1, 4 + attr_bonus(compute))
}

pub fn energy_at_level(compute: i32, level: i32) -> i32 {
    energy_per_level(compute) * level
}

pub fn skill_bonus(skill : Skill, skills: &Skills) -> i32 {
    if skills.skills.contains_key(&skill) {
        skills.skills[&skill]
    } else {
        -4
    }
}

