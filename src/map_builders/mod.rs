/* 
Modules are exposed through mod and pub mod; this file functions
as an interface - specifically, a list of what is provided by the module,
and how to interact with it. 
*/

use super::Map;

trait MapBuilder {
    fn build(new_depth: i32) -> Map;
}

pub fn build_random_map(new_depth: i32) -> Map {
    SimpleMapBuilder::build(new_depth)
}