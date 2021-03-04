use super::{BuilderChain, BuilderMap, InitialMapBuilder, TileType};
use std::collections::HashSet;

pub struct TownBuilder{}

pub fn town_builder(new_depth: i32, _rng: &mut rltk::RandomNumberGenerator, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height);
    chain.start_with(TownBuilder::new());
    chain
}

impl InitialMapBuilder for TownBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap) {
        self.build_rooms(rng, build_data);
    }
}

impl TownBuilder {
    pub fn new() -> Box<TownBuilder> {
        Box::new(TownBuilder{})
    }

    pub fn build_rooms(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap) {
        self.grass_layer(build_data);
        self.water_and_piers(rng, build_data);
        
        // Make visible for procgen snapshot/screenshot
        for t in build_data.map.visible_tiles.iter_mut() {
            *t = true;
        }
        build_data.take_snapshot();
    }

    fn grass_layer(&mut self, build_data : &mut BuilderMap) {
        // Let's lay down some soft organic matter
        for t in build_data.map.tiles.iter_mut() {
            *t = TileType::Grass;
        }
        build_data.take_snapshot();
    }

    fn water_and_piers(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap){
        let mut n = (rng.roll_dice(1, 65535) as f32) / 65535f32;
        let mut water_width : Vec<i32> = Vec::new();
        for y in 0..build_data.height {
            let n_water = (f32::sin(n) * 10.0) as i32 + 14 + rng.roll_dice(1, 6);
            water_width.push(n_water);
            n += 0.1;
            for x in 0..n_water+3 {
                let idx = build_data.map.xy_idx(x, y);
                build_data.map.tiles[idx] = TileType::ShallowWater;
            }
        }
        build_data.take_snapshot();

    // Add piers
    for _i in 0..rng.roll_dice(1, 4)+6 {
        let y = rng.roll_dice(1, build_data.height)-1;
        for x in 2 + rng.roll_dice(1, 6) .. water_width[y as usize] + 4 {
            let idx = build_data.map.xy_idx(x, y);
            build_data.map.tiles[idx] = TileType::WoodFloor;
            }
        }
    build_data.take_snapshot();
    }
}
