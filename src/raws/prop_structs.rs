use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Prop {
    pub name : String,
    pub renderable: Option<Renderable>,
    pub blocks_tile : bool,
    pub blocks_visibility : bool,
}

#[derive(Deserialize, Debug)]
pub struct Renderable {
    pub glyph: String,
    pub fg : String,
    pub bg : String,
    pub order : i32
}