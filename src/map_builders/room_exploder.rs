use super::{MetaMapBuilder, BuilderMap, TileType, paint, Symmetry, Rect};
use rltk::RandomNumberGenerator;

pub struct RoomExploder{}

impl MetaMapBuilder for RoomExploder {
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data : &mut BuilderMap {
        self.build(rng, build_data);
    }
}

impl RoomExploder {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomExploder> {
        Box::new(RoomExploder{})
    }

    
}