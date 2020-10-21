use rltk::{ RGB, RandomNumberGenerator };
use specs::prelude::*;
use super::{CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile, Rect, Item,
random_table::RandomTable, EquipmentSlot, Equippable, MeleePowerBonus, DefenseBonus, HungerClock,
Consumable, Ranged, ProvidesHealing, InflictsDamage, AreaOfEffect, Confusion, SerializeMe,
HungerState, ProvidesFood, MagicMapper, Hidden, EntryTrigger, SingleActivation, Map, TileType, Door, BlocksVisibility };
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

/// Spawns the player and returns his/her entity object.
pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 10, dirty: true })
        .with(Name{name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .with(HungerClock{ state: HungerState::WellFed, duration: 30 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

const MAX_MONSTERS : i32 = 4;

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Recyculon", 1)
        .add("Mopbot", 1 + map_depth)  
        .add("Scrambler Cell", 1 + map_depth)
        .add("Repair Pack", 4)
        .add("Incendiary Grenade", 2 + map_depth)
        .add("Beam Cell", 3 + map_depth)
        .add("Blade Effector", map_depth -1)
        .add("Plasteel Shard", map_depth)
        .add("Weak Defensive Effectors", map_depth)
        .add("Malfunctioning Defensive Effectors", map_depth - 1)
        .add("Rations", 10)
        .add("Data Disk - Map", 2)
        .add("Spike Trap", 5)
}

/// Fills a room with stuff!
pub fn spawn_room(map: &Map, rng: &mut RandomNumberGenerator, room : &Rect, map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
    let mut possible_targets : Vec<usize> = Vec::new();
    { // Borrow scope - to keep access to the map separated
        for y in room.y1 + 1 .. room.y2 {
            for x in room.x1 + 1 .. room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(map, rng, &possible_targets, map_depth, spawn_list);
}

/// Fills a region with stuff!
pub fn spawn_region(_map: &Map, rng: &mut RandomNumberGenerator, area : &[usize], map_depth: i32, spawn_list : &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points : HashMap<usize, String> = HashMap::new();
    let mut areas : Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns = i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 { return; }

        for _i in 0 .. num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (rng.roll_dice(1, areas.len() as i32)-1) as usize };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll(rng));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
pub fn spawn_entity(ecs: &mut World, spawn : &(&usize, &String)) {
    let map = ecs.fetch::<Map>();
    let width = map.width as usize;
    let x = (*spawn.0 % width) as i32;
    let y = (*spawn.0 / width) as i32;
    std::mem::drop(map);

    match spawn.1.as_ref() {
        "Recyculon" => recyculon(ecs, x, y),
        "Mopbot" => mopbot(ecs, x, y),
        "Repair Pack" => repair_pack(ecs, x, y),
        "Incendiary Grenade" => incendiary_grenade(ecs, x, y),
        "Scrambler Cell" => scrambler_cell(ecs, x, y),
        "Beam Cell" => beam_cell(ecs, x, y),
        "Plasteel Shard" => dagger(ecs, x, y),
        "Malfunctioning Defensive Effectors" => shield(ecs, x, y),
        "Blade Effector" => longsword(ecs, x, y),
        "Weak Defensive Effectors" => shield_lv2(ecs, x, y),
        "Rations" => rations(ecs, x, y),
        "Data Disk - Map" => magic_mapper(ecs, x, y),
        "Spike Trap" => spike_trap(ecs, x, y),
        "Door" => door(ecs, x, y),
        _ => {}
    }
}


fn mopbot(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('M'), "Mopulon"); }
fn recyculon(ecs: &mut World, x: i32, y: i32) { monster(ecs, x, y, rltk::to_cp437('R'), "Recyclobot"); }

/// BUILD A MONSTER! It's got the following components: Position, renderable, viewshed, Monster, Name, etc etc etc
fn monster<S : ToString>(ecs: &mut World, x: i32, y: i32, glyph : rltk::FontCharType, name : S) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1
        })
        .with(Viewshed{ visible_tiles : Vec::new(), range: 9, dirty: true })
        .with(Monster{})
        .with(Name{ name: name.to_string() }) //
        .with(BlocksTile{})
        .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn magic_mapper(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
    .with(Position{ x, y, })
    .with(Renderable{
        glyph: rltk::to_cp437(')'),
        fg: RGB::named(rltk::CYAN3),
        bg: RGB::named(rltk::BLACK),
        render_order: 2        
    })
    .with(Name{ name: "Data Disk - Map".to_string() })
    .with(Item{})
    .with(MagicMapper{})
    .with(Consumable{})
    .marked::<SimpleMarker<SerializeMe>>()
    .build();
}


fn rations(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('&'),
            fg: RGB::named(rltk::GREEN),
            bg:RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Rations".to_string() })
        .with(Item{})
        .with(ProvidesFood{})
        .with(Consumable{})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn dagger(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Plasteel Shard".to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Malfunctioning Defensive Effectors".to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn longsword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Blade Effector".to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn shield_lv2(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Weak Defensive Effectors".to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn repair_pack(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Repair pack".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing{ heal_amount: 12 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
fn incendiary_grenade(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Incendiary Grenade".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 20 })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn beam_cell(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Beam cell".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 10 })
        .with(AreaOfEffect{ radius: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn scrambler_cell(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Scrambler Cell".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(Confusion{ turns: 4 })
        .with(AreaOfEffect{ radius: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn spike_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('^'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 2
        })
        .with(Name{ name : "Spike Trap".to_string() })
        .with(Hidden{})
        .with(EntryTrigger{})
        .with(SingleActivation{})
        .with(InflictsDamage{ damage : 10 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    }

fn door(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
    .with(Position{ x, y })
    .with(Renderable{
        glyph: rltk::to_cp437('+'),
        fg: RGB::named(rltk::CHOCOLATE),
        bg: RGB::named(rltk::BLACK),
        render_order: 2
    })
    .with(Name{ name : "Door".to_string() })
    .with(BlocksTile{})
    .with(BlocksVisibility{})
    .with(Door{open: false})
    .marked::<SimpleMarker<SerializeMe>>()
    .build();
}