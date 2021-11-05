extern crate sdl2;

#[path = "./globals.rs"]
mod globals;

use Rect;

pub fn axis_aligned(collider_a: &Rect, collider_b: &Rect) -> bool {
    // Get comparision variables for first collider
    let top_a = collider_a.y;
    let right_a = collider_a.x + collider_a.width() as i32;
    let bottom_a = collider_a.y + collider_a.height() as i32;
    let left_a = collider_a.x;
    // Get comparision varaibles for second collider
    let top_b = collider_b.y;
    let right_b = collider_b.x + collider_b.width() as i32;
    let bottom_b = collider_b.y + collider_b.height() as i32;
    let left_b = collider_b.x;
    
    if bottom_a <= top_b ||
       top_a >= bottom_b ||
       right_a <= left_b ||
       left_a >= right_b {

        return false;
    }

    return true;
}

pub fn screen_boarder(collider: &Rect) -> bool {
    // Screen width check
    if collider.x < 0 || collider.x as u32 + collider.width() > globals::SCREEN_WIDTH {
        return true;
    }
    // Screen height check
    if collider.y < 0 || collider.y as u32 + collider.height() > globals::SCREEN_HEIGHT {
        return true;
    }
    // Return false otherwise
    return false;
}