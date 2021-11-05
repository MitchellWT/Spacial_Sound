extern crate sdl2;

use Player;
use Window;
use Canvas;
use Color;
use std::path::Path;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::mixer::Music;

pub struct AudioSource {
    music:    Music<'static>,
    collider: Rect,
    min_distance: u32,
    max_distance: u32,
}

impl AudioSource {
    // Function for creating a audio source struct
    pub fn new(x: i32, y: i32, width: u32, height: u32, str_path: &str, min_distance: u32, max_distance: u32) -> AudioSource {
        // Build path from string
        let path = Path::new(str_path);
        let raw = AudioSource {
            music: Music::from_file(path).unwrap(),
            collider: Rect::from_center(Point::new(x, y), width, height),
            min_distance: min_distance,
            max_distance: max_distance
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
    // Changes players internal values
    pub fn update(&self, player: &Player) -> bool {
        self.volume_check(player)
    }
    // Draws the audio source to the screen as a rect
    pub fn render(&self, canvas: &mut Canvas<Window>) -> bool {
        canvas.set_draw_color(Color::RGB(155, 0, 0));
        match canvas.fill_rect(self.collider()) {
            Ok(_)  => true,
            Err(_) => false
        }
    }
    //
    fn volume_check(&self, player: &Player) -> bool {
        let distance = self.between_distance(player.collider().x, player.collider().y);
        println!("{}", distance);
        true
    }
    // 
    fn between_distance(&self, player_x: i32, player_y: i32) -> f64 {
        // Calculate delta x
        let mut delta_x: f64 = (self.collider.x - player_x).into();
        delta_x = delta_x * delta_x;
        // Calculate delta y
        let mut delta_y: f64 = (self.collider.y - player_y).into();
        delta_y = delta_y * delta_y;
        
        let distance = delta_x + delta_y;
        return distance.sqrt();
    }
}
