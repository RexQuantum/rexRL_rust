use serde::{Deserialize};
use std::{collections::HashMap};
use super::{Renderable};

#[derive(Deserialize, Debug)]
pub struct Mob {
    pub name : String,
    pub renderable : Option<Renderable>,
    pub blocks_tile : bool,
    pub vision_range : i32,
    pub ai : String,
    pub quips : Option<Vec<String>>,
    pub attributes : MobAttributes,
    pub skills : Option<HashMap<String, i32>>,
    pub level : Option<i32>,
    pub hp : Option<i32>,
    pub energy : Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct MobAttributes {
    pub strength : Option<i32>,
    pub quickness : Option<i32>,
    pub integrity : Option<i32>,
    pub compute : Option<i32>
}