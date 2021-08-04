use super::{Map, Rect, TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use specs::prelude::*;
mod simple_map;
mod bsp_dungeon;
mod bsp_interior;
mod cellular_automata;
mod drunkard;
mod maze;
mod dla;
mod common;
mod voronoi;
mod waveform_collapse;
mod prefab_builder;
mod room_based_spawner;
mod room_based_starting_position;
mod room_based_stairs;
mod area_starting_points;
mod cull_unreachable;
mod voronoi_spawning;
mod distant_exit;
mod room_exploder;
mod room_corner_rounding;
mod rooms_corridors_dogleg;
mod rooms_corridors_bsp;
mod room_sorter;
mod room_draw;
mod rooms_corridors_nearest;
mod rooms_corridors_lines;
mod room_corridor_spawner;
mod door_placement;
mod town;
use distant_exit::DistantExit;
use simple_map::SimpleMapBuilder;
use bsp_dungeon::BspDungeonBuilder;
use bsp_interior::BspInteriorBuilder;
use cellular_automata::CellularAutomataBuilder;
use drunkard::DrunkardsWalkBuilder;
use voronoi::VoronoiCellBuilder;
use waveform_collapse::WaveformCollapseBuilder;
use prefab_builder::PrefabBuilder;
use room_based_spawner::RoomBasedSpawner;
use room_based_starting_position::RoomBasedStartingPosition;
use room_based_stairs::RoomBasedStairs;
use area_starting_points::{AreaStartingPosition, XStart, YStart};
use cull_unreachable::CullUnreachable;
use voronoi_spawning::VoronoiSpawning;
use maze::MazeBuilder;
use dla::DLABuilder;
use common::*;
use room_exploder::RoomExploder;
use room_corner_rounding::RoomCornerRounder;
use rooms_corridors_dogleg::DoglegCorridors;
use rooms_corridors_bsp::BspCorridors;
use room_sorter::{RoomSorter, RoomSort};
use room_draw::RoomDrawer;
use rooms_corridors_nearest::NearestCorridors;
use rooms_corridors_lines::StraightLineCorridors;
use room_corridor_spawner::CorridorSpawner;
use door_placement::*;
use town::town_builder;

// This is our shared map state. BuilderMap is used when we want to pass
// data between different map builders, rather than each map builder defining
// its own copies of shared data
pub struct BuilderMap {
    pub spawn_list : Vec<(usize, String)>,
    pub map : Map,
    pub starting_position : Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history : Vec<Map>,
    pub width: i32,
    pub height: i32
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

//  Previously, we've passed MapBuilder classes around, each capable of building previous maps.
//  Since we've concluded that this is a poor idea, and defined the syntax we want, we'll make a replacement.
//  The BuilderChain is a master builder - it controls the whole build process. To this end, we'll add the BuilderChain type:
pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data : BuilderMap
}

impl BuilderChain {
    pub fn new(new_depth : i32, width: i32, height: i32) -> BuilderChain {
        BuilderChain{
            starter: None,
            builders: Vec::new(),
            build_data : BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height),
                starting_position: None,
                rooms: None,
                corridors: None,
                history : Vec::new(),
                width,
                height
            }
        }
    }

    pub fn start_with(&mut self, starter : Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }

// Simply add the meta-builder to the builder vector. Since vectors remain
// in the order in which you add to them,
// the operations will remain sorted appropriately.
    pub fn with(&mut self, metabuilder : Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

// Finally, we'll implement a function to actually construct the map:
    pub fn build_map(&mut self, rng : &mut rltk::RandomNumberGenerator) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(rng, &mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(rng, &mut self.build_data);
        }
    }
    // Walk through the spawn list to spawn entities.
    pub fn spawn_entities(&mut self, ecs : &mut World) {
        for entity in self.build_data.spawn_list.iter() {
            spawner::spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap);
}

fn random_start_position(rng: &mut rltk::RandomNumberGenerator) -> (XStart, YStart) {
    let x;
    let xroll = rng.roll_dice(1, 3);
    match xroll {
        1 => x = XStart::LEFT,
        2 => x = XStart::CENTER,
        _ => x = XStart::RIGHT
    }

    let y;
    let yroll = rng.roll_dice(1, 3);
    match yroll {
        1 => y = YStart::BOTTOM,
        2 => y = YStart::CENTER,
        _ => y = YStart::TOP
    }

    (x, y)
}

fn random_room_builder(rng: &mut rltk::RandomNumberGenerator, builder : &mut BuilderChain) {
    let build_roll = rng.roll_dice(1, 3);
    match build_roll {
        1 => { builder.start_with(SimpleMapBuilder::new());
            rltk::console::log(format!("Base Room Type: Simple Map"))}
        2 => { builder.start_with(BspDungeonBuilder::new());
            rltk::console::log(format!("Base Room Type: BSP - Dungeon variant"))}
        _ => { builder.start_with(BspInteriorBuilder::new());
            rltk::console::log(format!("Base Room Type: BSP - interior "))}
    }

    // BSP Interior still makes holes in the walls
    if build_roll != 3 {
        // Sort by one of the 5 available algorithms
        let sort_roll = rng.roll_dice(1, 5);
        match sort_roll {
            1 => builder.with(RoomSorter::new(RoomSort::LEFTMOST)),
            2 => builder.with(RoomSorter::new(RoomSort::RIGHTMOST)),
            3 => builder.with(RoomSorter::new(RoomSort::TOPMOST)),
            4 => builder.with(RoomSorter::new(RoomSort::BOTTOMMOST)),
            _ => builder.with(RoomSorter::new(RoomSort::CENTRAL)),
        }

        builder.with(RoomDrawer::new());

        let corridor_roll = rng.roll_dice(1, 4);
        //rltk::console::log(format!("corridor_roll - 1-dogleg/2-nearest/3-strline/4-bsp = [{}]", corridor_roll ));
        match corridor_roll {
            1 => { builder.with(DoglegCorridors::new());
            rltk::console::log(format!("Corridor type: Dogleg"))}
            2 => { builder.with(NearestCorridors::new());
                rltk::console::log(format!("Corridor type: Nearest"))}
            3 => { builder.with(StraightLineCorridors::new());
                rltk::console::log(format!("Corridor type: Straight Line Corridors"))}
            _ => { builder.with(BspCorridors::new());
                rltk::console::log(format!("Corridor type: BSP"))}
        }

            let cspawn_roll = rng.roll_dice(1, 2);
            if cspawn_roll == 1 {
                builder.with(CorridorSpawner::new());
                rltk::console::log(format!("Let's put some baddies in the corridors"));
            }

        let modifier_roll = rng.roll_dice(1, 6);
        match modifier_roll {
            1 => { builder.with(RoomExploder::new());
                rltk::console::log(format!("Let's carve out these rooms, they're boring"))}
            2 => { builder.with(RoomCornerRounder::new());
                rltk::console::log(format!("Let's round the corners of these rooms"))}
            _ => {}
        }
    }

    let start_roll = rng.roll_dice(1, 2);
    match start_roll {
        1 => builder.with(RoomBasedStartingPosition::new()),
        _ => {
            let (start_x, start_y) = random_start_position(rng);
            builder.with(AreaStartingPosition::new(start_x, start_y));
        }
    }

    let exit_roll = rng.roll_dice(1, 2);
    match exit_roll {
        1 => builder.with(RoomBasedStairs::new()),
        _ => builder.with(DistantExit::new())
    }

    let spawn_roll = rng.roll_dice(1, 2);
    match spawn_roll {
        1 => builder.with(RoomBasedSpawner::new()),
        _ => builder.with(VoronoiSpawning::new())
    }
}

fn random_shape_builder(rng: &mut rltk::RandomNumberGenerator, builder : &mut BuilderChain) {
    let builder_roll = rng.roll_dice(1, 14);
    match builder_roll {
        1 => { builder.start_with(CellularAutomataBuilder::new());
            rltk::console::log(format!("Builder: Cellular Automata"))},
        2 => { builder.start_with(DrunkardsWalkBuilder::open_area());
            rltk::console::log(format!("Builder: Drunkards Walk — open area variant"))},
        3 => { builder.start_with(DrunkardsWalkBuilder::open_halls());
            rltk::console::log(format!("Builder: Drunkards Walk — open halls variant"))},
        4 => { builder.start_with(DrunkardsWalkBuilder::winding_passages());
            rltk::console::log(format!("Builder: Drunkards Walk — winding passages variant"))}
        5 => { builder.start_with(DrunkardsWalkBuilder::fat_passages());
            rltk::console::log(format!("Builder: Drunkards Walk — fat passages variant"))}
        6 => { builder.start_with(DrunkardsWalkBuilder::fearful_symmetry());
            rltk::console::log(format!("Builder: Drunkards Walk — fearful symmetry"))}
        7 => { builder.start_with(DLABuilder::walk_inwards());
            rltk::console::log(format!("Builder: Diffusion-Limited Aggregation — walk inwards"))}
        8 => { builder.start_with(DLABuilder::walk_outwards());
            rltk::console::log(format!("Builder: Diffusion-Limited Aggregation — walk outwards"))}
        9 => { builder.start_with(DLABuilder::central_attractor());
            rltk::console::log(format!("Builder: Diffusion-Limited Aggregation — central attractor"))}
        10 =>{ builder.start_with(DLABuilder::insectoid());
            rltk::console::log(format!("Builder: Diffusion-Limited Aggregation — insectoid"))}
        11 =>{ builder.start_with(VoronoiCellBuilder::pythagoras());
            rltk::console::log(format!("Builder: Voronoi Cell —Pythagoras algorithm"))}
        12 =>{ builder.start_with(MazeBuilder::new());
            rltk::console::log(format!("Builder: It's a MAAAAAZE!"))}
        13 => { builder.start_with(VoronoiCellBuilder::manhattan());
            rltk::console::log(format!("Builder: Voronoi Cell - Manhattan algorithm"))}
        _ => builder.start_with(PrefabBuilder::constant(prefab_builder::prefab_levels::WFC_POPULATED)),
    }

    // Set the start to the center and cull
    builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    builder.with(CullUnreachable::new());

    // Now set the start to a random starting area
    let (start_x, start_y) = random_start_position(rng);
    builder.with(AreaStartingPosition::new(start_x, start_y));

    // Setup an exit and spawn mobs
    builder.with(VoronoiSpawning::new());
    builder.with(DistantExit::new());
}

pub fn random_builder(new_depth: i32, rng: &mut rltk::RandomNumberGenerator, width: i32, height: i32) -> BuilderChain {
    let mut builder = BuilderChain::new(new_depth, width, height);
    let type_roll = rng.roll_dice(1, 2);
    match type_roll {
        1 => {random_room_builder(rng, &mut builder);
            rltk::console::log(format!("Rooms or shapes? ROOMS!"))}
        _ => {random_shape_builder(rng, &mut builder);
            rltk::console::log(format!("Rooms or shapes? SHAPES!"))}
    }

    if rng.roll_dice(1, 3)==1 {
    builder.with(WaveformCollapseBuilder::new());
    rltk::console::log(format!("Commencing Waveform Collapse"));

        // Now set the start to a random starting area
        let (start_x, start_y) = random_start_position(rng);

        builder.with(AreaStartingPosition::new(start_x, start_y));

        // Setup an exit and spawn mobs
        builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }

    if rng.roll_dice(1, 20)==1 {
        builder.with(PrefabBuilder::sectional(prefab_builder::prefab_sections::UNDERGROUND_FORT));
    }

    builder.with(DoorPlacement::new());
    builder.with(PrefabBuilder::vaults());

    builder
}

pub fn level_builder(new_depth: i32, rng: &mut rltk::RandomNumberGenerator, width: i32, height: i32) -> BuilderChain {
    rltk::console::log(format!("Depth: {}", new_depth));
    match new_depth {
        1 => town_builder(new_depth, rng, width, height),
        _ => random_builder(new_depth, rng, width, height)
    }
}