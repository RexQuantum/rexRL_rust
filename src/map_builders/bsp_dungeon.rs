pub struct BspDungeonBuilder {
    map : Map,
    starting_position : Position,
    depth: i32,
    rooms: Vec<Rect>,
    history: Vec<Map>,
    rects: Vec<Rect>
}

impl MapBuilder for BspDungeonBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self) {
        // I should do something here lol. once I figure out BSP maps.
    }

    fn spawn_etities(&mut self, ecs : &mut World) {
        for room in self.room.iter().skip(1) {
            spawner::spawn_room(ecs, room, self.depth);
        }
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

impl BspDungeonBuilder {
    pub fn new(new_depth : i32) -> BspDungeonBuilder {
        BspDungeonBuilder{
            map : Map::new(new_depth),
            starting_position : Position{ x: 0, y : 0 },
            depth : new_depth,
            rooms: Vec::new(),
            history: Vec::new(),
            rects: Vec::new()
        }
    }
    
fn build(&mut self) {
    let mut rng = RandomNumberGenerator::new();

    self.rects.clear();
    self.rects.push( Rect::new(2, 2, self.map.width-5, self.map.height-5) ); // Start with a single map-sized rectangle
    let first_room = self.rects[0];
    self.add_subrects(first_room); // Divide the first room

    // Up to 240 times, we get a random rectangle and divide it. If its possible to squeeze a
    // room in there, we place it and add it to the rooms list.
    let mut n_rooms = 0;
    while n_rooms < 240 {
        let rect = self.get_random_rect(&mut rng);
        let candidate = self.get_random_sub_rect(rect, &mut rng);

        if self.is_possible(candidate) {
            apply_room_to_map(&mut self.map, &candidate);
            self.rooms.push(candidate);
            self.add_subrects(rect);
            self.take_snapshot();
        }

        n_rooms += 1;
    }
    let start = self.rooms[0].center();
    self.starting_position = Position{ x: start.0, y: start.1 };
    }
}
