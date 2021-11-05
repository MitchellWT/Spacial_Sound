extern crate sdl2;

use sdl2::mixer::Chunk;
use sdl2::mixer::Channel;
use Player;
use Window;
use Canvas;
use Color;
use std::path::Path;
use sdl2::rect::Point;
use sdl2::rect::Rect;

pub struct AudioSource {
    audio:        Chunk,
    channel:      Channel,
    collider:     Rect,
    min_distance: u32,
    max_distance: u32,
}

impl AudioSource {
    // Function for creating a audio source struct
    pub fn new(x: i32, y: i32, width: u32, height: u32, str_path: &str, min_distance: u32, max_distance: u32) -> AudioSource {
        // Build path from string
        let path = Path::new(str_path);
        let raw = AudioSource {
            audio:        Chunk::from_file(path).unwrap(),
            channel:      Channel::all(),
            collider:     Rect::from_center(Point::new(x, y), width, height),
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
        match self.channel.play(&self.audio, -1) {
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
    // Checks player distance to audio source for audio source volume
    fn volume_check(&self, player: &Player) -> bool {
        // Gets distance to player
        let distance = self.between_distance(player.collider().x, player.collider().y);
        // Sets volume to zero If the player is out of the audio source range
        if distance as u32 >= self.max_distance {
            self.channel.set_volume(0);
        // Sets volume to max If the player is close to the audio source
        } else if distance as u32 <= self.min_distance {
            self.channel.set_volume(128);
        } else {
            // Normalises the distance between 0 and 128
            let normalised_distance = (
                                        (distance - self.min_distance as f64) /
                                        (self.max_distance as f64 - self.min_distance as f64)
                                      ) * 128.0;
            // Inverses the normalised distance
            let inversed_distance = (normalised_distance as i32 - 128).abs();
            
            self.channel.set_volume(inversed_distance);
        }

        true
    }
    // Gets the distance between the player and the audio source
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
