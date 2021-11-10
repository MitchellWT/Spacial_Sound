extern crate sdl2;

#[path = "../globals.rs"]
mod globals;

use Direction;
use Rect;

pub fn axis_aligned(collider_a: &Rect, collider_b: &Rect) -> bool {
    // Get comparision variables for first collider
    let top_a    = collider_a.y;
    let right_a  = collider_a.x + collider_a.width() as i32;
    let bottom_a = collider_a.y + collider_a.height() as i32;
    let left_a   = collider_a.x;
    // Get comparision varaibles for second collider
    let top_b    = collider_b.y;
    let right_b  = collider_b.x + collider_b.width() as i32;
    let bottom_b = collider_b.y + collider_b.height() as i32;
    let left_b   = collider_b.x;

    if bottom_a <= top_b ||
       top_a >= bottom_b ||
       right_a <= left_b ||
       left_a >= right_b {

        return false;
    }

    return true;
}
// WARN: The below direction calculation can have issues with rectangles should only
// be used with squares

// TODO: Fix direction calculation to work with rectangles, potentially add the collider
// width/height to the equation?

// The calculations are relative to the screen not the player rotation
pub fn axis_aligned_direction(collider_a: &Rect, collider_b: &Rect) -> Direction {
    // Get colliders center    
    let center_a = collider_a.center();
    let center_b = collider_b.center();
    // Get horizontal and vertical distance from collider center
    let horizontal_distance = center_a.x - center_b.x;
    let vertical_distance   = center_a.y - center_b.y;
    // Check which axis the collision occured on
    if horizontal_distance.abs() > vertical_distance.abs() {
        // Collision check for right direction
        if horizontal_distance < 0 {
            return Direction::E;
        }
        // Collision check for left direction
        else {
            return Direction::W;
        }
    } else {
        // Collision check for down direction
        if vertical_distance < 0 {
            return Direction::S;
        // Collision check for up direction
        } else {
            return Direction::N;
        }
    }
}

pub fn screen_boarder(collider: &Rect) -> (Direction, Direction) {
    let mut first_direction = Direction::NULL;
    // Screen left side check
    if collider.x < 0 {
        first_direction = Direction::W;
    }
    // Screen right side check
    if collider.x + collider.width() as i32 > globals::SCREEN_WIDTH as i32 {
        if first_direction != Direction::NULL {
            return (first_direction, Direction::E);
        }

        first_direction = Direction::E;
    }
    // Screen top side check
    if collider.y < 0 {
        if first_direction != Direction::NULL {
            return (first_direction, Direction::N);
        }
        
        first_direction = Direction::N;
    }
    // Screen bottom side check
    if collider.y + collider.height() as i32 > globals::SCREEN_HEIGHT as i32 {
        if first_direction != Direction::NULL {
            return (first_direction, Direction::S);
        }
        
        first_direction = Direction::S;
    }
    if first_direction != Direction::NULL {
        return (first_direction, Direction::NULL);
    }

    // Return Direction::NULL otherwise
    return (Direction::NULL, Direction::NULL);
}
// This calculation is inaccurate with two moving bodies
// but will be correct with one moving body
pub fn axis_aligned_continous(new_collider: &mut Rect, direction: &Direction, collider: &Rect) -> bool {
    // Check for inital collision, when player is against collider
    let mut collided = axis_aligned(&new_collider, collider);
    if collided {return false}

    // Increments collider until collision occurs
    while !collided {
        match *direction {
            Direction::N  => {new_collider.set_y(new_collider.y() - 1)},
            Direction::E  => {new_collider.set_x(new_collider.x() + 1)},
            Direction::S  => {new_collider.set_y(new_collider.y() + 1)},
            Direction::W  => {new_collider.set_x(new_collider.x() - 1)},
            _ => {}   
        }

        collided = axis_aligned(&new_collider, collider);
    }

    true
}