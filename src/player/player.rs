extern crate sdl2;

#[path = "./direction.rs"]
mod direction;
#[path = "../globals.rs"]
mod globals;

use Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::{Point, Rect};
use direction::Direction;

pub struct Player {
    collider: Rect,
    velocity: i32,
}

impl Player {
    // Function for creating a player struct
    pub fn new(x: i32, y: i32, width: u32, height: u32, velocity: i32) -> Player {
        let raw = Player {
            collider: Rect::from_center(Point::new(x, y), width, height),
            velocity: velocity
        };
        
        raw
    }
    // Get collider, not mutable
    pub fn collider(&self) -> Rect {
        self.collider
    }
    // Get mutable collider
    pub fn mut_collider(&mut self) -> &mut Rect {
        &mut self.collider
    }
    // Get position of left ear, point struct
    pub fn getLeftEar(&self) -> Point {
        return Point::new(self.collider.left(), self.collider.center().y());
    }
    // Get posiiton of right ear, point struct
    pub fn getRightEar(&self) -> Point {
        return Point::new(self.collider.right(), self.collider.center().y());
    }
    // Get velocity, not mutable
    pub fn velocity(&self) -> i32 {
        self.velocity
    }
    // Changes players internal values
    pub fn update(&mut self, direction: &Direction) -> bool {
        // For all re-assignment values a 'new' variable is
        // created. This was due to the mutable self reference
        match direction {
            Direction::N  => {
                let new_y = self.collider().y() - self.velocity();
                self.mut_collider().set_y(new_y);
            },
            Direction::NE => {},
            Direction::E  => {
                let new_x = self.collider().x() + self.velocity();
                self.mut_collider().set_x(new_x);
            },
            Direction::SE => {},
            Direction::S  => {
                let new_y = self.collider().y() + self.velocity();
                self.mut_collider().set_y(new_y);
            },
            Direction::SW => {},
            Direction::W  => {
                let new_x = self.collider().x() - self.velocity();
                self.mut_collider().set_x(new_x);
            },
            Direction::NW => {},
            Direction::NULL => {}   
        }
        
        true
    }
    // Draws the player to the screen, should be called after update
    pub fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.collider()) {
            Ok(_)  => true,
            Err(_) => false
        }
    }
}
