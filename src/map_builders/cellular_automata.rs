use super::{MapBuilder, Map, Rect, apply_room_to_map, 
    TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use rltk::RandomNumberGenerator;
use specs::prelude::*;

const MIN_ROOM_SIZE : i32 = 8;

pub struct CellularAutomataBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    history: Vec<Map>
}

impl MapBuilder for CellularAutomataBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self)  {
        //self.build(); - we should write this
    }

    fn spawn_entities(&mut self, ecs : &mut World) {
        // We need to rewrite this, too.
    }

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

impl CellularAutomataBuilder {
    pub fn new(new_depth : i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            history: Vec::new(),
        }
    }
}

fn build(&mut self) {
    let mut rng = RandomNumberGenerator::new();

    // First we completely randomize the map, setting 55% of it to be floor.
    for y in 1..self.map.height-1 {
        for x in 1..self.map.width-1 {
            let roll = rng.roll_dice(1, 100);
            let idx = self.map.xy_idx(x, y);
            if roll > 55 { self.map.tiles[idx] = TileType::Floor } 
            else { self.map.tiles[idx] = TileType::Wall }
        }
    }
    self.take_snapshot();

    // Now we iteratively apply cellular automata rules
    for _i in 0..15 {
        let mut newtiles = self.map.tiles.clone();

        for y in 1..self.map.height-1 {
            for x in 1..self.map.width-1 {
                let idx = self.map.xy_idx(x, y);
                let mut neighbors = 0;
                if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                if neighbors > 4 || neighbors == 0 {
                    newtiles[idx] = TileType::Wall;
                }
                else {
                    newtiles[idx] = TileType::Floor;
                }
            }
        }

        self.map.tiles = newtiles.clone();
        self.take_snapshot();
    }
}