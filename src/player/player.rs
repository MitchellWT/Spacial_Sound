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
    pub fn new(x: i32, y: i32, width: u32, height: u32, velocity: i32) -> Player {
        let raw = Player {
            collider: Rect::from_center(Point::new(x, y), width, height),
            velocity: velocity
        };
        return raw
    }

    pub fn collider(&self) -> Rect {
        self.collider
    }

    pub fn mut_collider(&mut self) -> &mut Rect {
        &mut self.collider
    }

    pub fn velocity(&self) -> i32 {
        self.velocity
    }

    pub fn update(&mut self, direction: &Direction) -> bool {
        match direction {
            Direction::N  => {
                let mut new_y = self.mut_collider().y();
                new_y = new_y - self.velocity();
                self.mut_collider().set_y(new_y);
            },
            Direction::NE => {},
            Direction::E  => {
                let mut new_x = self.mut_collider().x();
                new_x = new_x + self.velocity();
                self.mut_collider().set_x(new_x);
            },
            Direction::SE => {},
            Direction::S  => {
                let mut new_y = self.mut_collider().y();
                new_y = new_y + self.velocity();
                self.mut_collider().set_y(new_y);
            },
            Direction::SW => {},
            Direction::W  => {
                let mut new_x = self.mut_collider().x();
                new_x = new_x - self.velocity();
                self.mut_collider().set_x(new_x);
            },
            Direction::NW => {},
            Direction::NULL => {}   
        }
        
        true
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        match canvas.fill_rect(self.collider()) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
