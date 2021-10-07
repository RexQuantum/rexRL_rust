use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name : String,
    pub renderable : Option<Renderable>,
    pub consumable : Option<Consumable>,
    pub weapon : Option<Weapon>,
    pub shield : Option<Shield>
    // pub chestarmor : Option<ChestArmor>,
    // pub headarmor : Option<HeadArmor>,
    // pub legarmor : Option<LegArmor>,
    // pub handarmor : Option<HandArmor>
}

#[derive(Deserialize, Debug)]
pub struct Renderable {
    pub glyph: String,
    pub fg : String,
    pub bg : String,
    pub order : i32
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects : HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct Weapon {
    pub range: String,
    pub attribute: String,
    pub base_damage: String,
    pub skill_hit_bonus: i32
}

#[derive(Deserialize, Debug)]
pub struct Shield {
    pub defense_bonus: i32
}