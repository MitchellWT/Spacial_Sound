extern crate sdl2;

use Window;
use Canvas;
use Color;
use std::path::Path;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::mixer::Music;

pub struct AudioSource {
    music:    Music<'static>,
    collider: Rect
}

impl AudioSource {
    // Function for creating a audio source struct
    pub fn new(x: i32, y: i32, width: u32, height: u32, str_path: &str) -> AudioSource {
        // Build path from string
        let path = Path::new(str_path);
        let raw = AudioSource {
            music: Music::from_file(path).unwrap(),
            collider: Rect::from_center(Point::new(x, y), width, height)
        };
        
        raw
    }
    // Get collider, not mutable
    pub fn collider(&self) -> Rect {
        self.collider
    }
    // Plays music from audio source, infinite loop
    pub fn play(&self) -> bool {
        match self.music.play(-1) {
            Ok(_) => true,
            Err(_) => false
        }
    }
    // Draws the audio source to the screen as a rect
    pub fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(155, 0, 0));
        match canvas.fill_rect(self.collider()) {
            Ok(_)  => true,
            Err(_) => false
        }
    }
}
