use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};


// STRUCTURES! Building blocks! Like classes, and can store data or attach functions to.
// These "derive" macros tag components to save us lots of typing.
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range : i32,
    pub dirty : bool
}
