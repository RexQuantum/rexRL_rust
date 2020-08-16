use specs::prelude::*;
use super::{Viewshed, Monster, Name};
use rltk::{Point, console};

// "Monster," of course, is only a loosely-used term. It is debatable whether or not 
// any entities, hostile or no, are or aren't monsters, in the ethical sense. -Rex, 8.15.2020
pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);
                        
    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewshed, monster, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
                if viewshed.visible_tiles.contains(&*player_pos) {
                    console::log(&format!("The {} turns toward you and beeps angrily", name.name));
            }
        }
    }
}