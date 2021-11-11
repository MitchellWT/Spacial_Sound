extern crate sdl2;

use std::any::Any;
use player::entity::Entity;
use Direction;
use Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::{Point, Rect};

pub struct Player {
    id      : u32,
    collider: Rect,
    velocity: i32,
}

impl Entity for Player {
    // Needed for downcasting
    fn as_any(&self) -> &dyn Any {
        self
    }
    // Gets Id for entity
    fn id(&self) -> u32 {
        self.id
    }
    // Getter for collider, not mutable
    fn collider(&self) -> Rect {
        self.collider
    }    
    // Draws the player to the screen, should be called after update
    fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.collider()) {
            Ok(_)  => true,
            Err(_) => false
        }
    }
}

impl Player {
    // Function for creating a player struct
    pub fn new(id: u32, x: i32, y: i32, width: u32, height: u32, velocity: i32) -> Player {
        let raw = Player {
            id:       id,
            collider: Rect::from_center(Point::new(x, y), width, height),
            velocity: velocity
        };
        
        raw
    }
    // Getter for mutable collider
    pub fn mut_collider(&mut self) -> &mut Rect {
        &mut self.collider
    }
    // Setter for collider
    pub fn set_collider(&mut self, collider: Rect) {
        self.collider = collider
    }
    // Get position of left ear, point struct
    pub fn get_left_ear(&self) -> Point {
        return Point::new(self.collider.left(), self.collider.center().y());
    }
    // Get posiiton of right ear, point struct
    pub fn get_right_ear(&self) -> Point {
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
            Direction::E  => {
                let new_x = self.collider().x() + self.velocity();
                self.mut_collider().set_x(new_x);
            },
            Direction::S  => {
                let new_y = self.collider().y() + self.velocity();
                self.mut_collider().set_y(new_y);
            },
            Direction::W  => {
                let new_x = self.collider().x() - self.velocity();
                self.mut_collider().set_x(new_x);
            },
            _ => {}   
        }
        
        true
    }
}
