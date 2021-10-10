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
    Bridge,
    Gravel
}


/// Determines whether or not a tile is walkable and then returns a bool
pub fn tile_walkable(tt : TileType) -> bool {
    match tt {
        TileType::Floor | TileType::DownStairs | TileType::Grass |
        TileType::ShallowWater | TileType::WoodFloor | TileType::Bridge | TileType::Gravel | TileType::Road
        => true,
        _ => false
    }
}

/// Determines whether or not a tile is opaque and then returns a bool

pub fn tile_opaque(tt : TileType) -> bool {
    match tt {
        TileType::Wall => true,
        _ => false
    }
}

/// Matches TileType to its cost and returns an f32
pub fn tile_cost(tt : TileType) -> f32 {
    match tt {
        TileType::Road => 0.8,
        TileType::Grass => 1.1,
        TileType::ShallowWater => 1.2,
        _ => 1.0
    }
}