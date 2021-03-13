use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    Road,
    Grass,
    ShallowWater,
    DeepWater,
    WoodFloor,
<<<<<<< HEAD
    Bridge
=======
    Bridge,
    Gravel
>>>>>>> cbbafa4536ac4cc34a245e60b47b959c27bc3765
}

pub fn tile_walkable(tt : TileType) -> bool {
    match tt {
        TileType::Floor | TileType::DownStairs | TileType::Grass |
<<<<<<< HEAD
        TileType::ShallowWater | TileType::WoodFloor | TileType::Bridge
=======
        TileType::ShallowWater | TileType::WoodFloor | TileType::Bridge | TileType::Gravel
>>>>>>> cbbafa4536ac4cc34a245e60b47b959c27bc3765
            => true,
        _ => false        
    }
}

pub fn tile_opaque(tt : TileType) -> bool {
    match tt {
        TileType::Wall => true,
        _ => false
    }
}

pub fn tile_cost(tt : TileType) -> f32 {
    match tt {
        TileType::Road => 0.8,
        TileType::Grass => 1.1,
        TileType::ShallowWater => 1.2,
        _ => 1.0
    }
}