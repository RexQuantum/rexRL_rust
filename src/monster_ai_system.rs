use specs::prelude::*;
use super::{Viewshed, Monster, Map, Position, WantsToMelee, RunState, Confusion, particle_system::ParticleBuilder, 
            EntityMoved, gamelog::GameLog, Quips, Name};
use rltk::{ Point };


pub struct MonsterAI {}


impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>,
                        WriteStorage<'a, Confusion>,
                        WriteExpect<'a, ParticleBuilder>,
                        WriteStorage<'a, EntityMoved>,
                        WriteExpect<'a, rltk::RandomNumberGenerator>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, Quips>,
                        ReadStorage<'a, Name>);


    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, monster, mut position,
             mut wants_to_melee, mut confused, mut particle_builder, mut entity_moved, mut rng, mut gamelog, mut quips, names) = data;


        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed,_monster,mut pos) in (&entities, &mut viewshed, &monster, &mut position).join() {
            // Possibly quip
            let quip = quips.get_mut(entity);
            if let Some(quip) = quip {
                if !quip.available.is_empty() && viewshed.visible_tiles.contains(&player_pos) && rng.roll_dice(1,6)==1 {
                    let name = names.get(entity);
                    let quip_index = if quip.available.len() == 1 { 0 } else { (rng.roll_dice(1, quip.available.len() as i32)-1) as usize };
                    gamelog.entries.push(
                        format!("{} says \"{}\"", name.unwrap().name, quip.available[quip_index])
                    );
                    quip.available.remove(quip_index);
                }
            }

            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;

                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA),
                                    rltk::RGB::named(rltk::BLACK), rltk::to_cp437('?'), 200.0);
            }


            if can_act {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
                }
                else if viewshed.visible_tiles.contains(&*player_pos) {
                    // Path to the player
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y),
                        map.xy_idx(player_pos.x, player_pos.y),
                        &*map
                    );
                    if path.success && path.steps.len()>1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        entity_moved.insert(entity, EntityMoved{}).expect("Unable to insert marker");
                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}