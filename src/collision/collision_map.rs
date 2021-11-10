use Direction;
use std::collections::HashMap;

pub struct CollisionMap {
    collision_map: HashMap<u32, Direction>
}

impl CollisionMap {
    // Function for creating a new collision map
    pub fn new() -> CollisionMap {
        let raw = CollisionMap {
            collision_map: HashMap::new()
        };

        raw
    }
    // Getter for direction, using id
    pub fn get_direction(&self, id: u32) -> Option<&Direction> {
        return self.collision_map.get(&id);
    }
    // Setter for direction, using id
    pub fn set_direction(&mut self, id: u32, direction: Direction) {
        self.collision_map.insert(id, direction);
    }
    // Checks If a direction appears in the collision map
    pub fn check_for_direction(&self, direction: &Direction) -> bool {
        self.collision_map.values().any(|val| *val == *direction)
    }
}
