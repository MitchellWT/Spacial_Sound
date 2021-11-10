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
    pub fn get_first_id(&self, direction: &Direction) -> &u32 {
        // Reference: https://stackoverflow.com/questions/59401720/how-do-i-find-the-key-for-a-value-in-a-hashmap
        self.collision_map.iter().find_map(|(key, val)| if val == direction {Some(key)} else {None}).unwrap()
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
    // Gets all collided id's
    pub fn get_collided(&self) -> Vec<u32> {
        let mut collision_vec: Vec<u32> = Vec::new();
        self.collision_map.iter().for_each(|(key, val)| {
            if *val != Direction::NULL {
                collision_vec.push(*key);
            }
        });

        collision_vec
    }
}
