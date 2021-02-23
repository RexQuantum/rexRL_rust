use super::{BuilderChain, BuilderMap, InitialMapBuilder, TileType, Position};
use std::collections::HashSet;


pub fn level_builder(new_depth: i32, rng: &mut rltk::RandomNumberGenerator, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height);
    chain.start_with(TownBuilder::new());
    let (start_x, start_y) = super::random_start_position(rng);
    chain.with(AreaStartingPosition::new(start_x, start_y));
    chain.with(DistantExit::new());
    chain
}

pub struct TownBuilder{}

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

    
}
