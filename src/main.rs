extern crate sdl2;

#[path = "./player/player.rs"]
mod player;
#[path = "./player/direction.rs"]
mod direction;
#[path = "./audio/audio_source.rs"]
mod audio_source;

mod globals;

use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::{thread, time};

use player::Player;
use direction::Direction;
use audio_source::AudioSource;

fn main() {
    // Sets up SDL and get required variables for
    // operations
    let setup_tuple    = sdl_setup();
    let mut canvas     = setup_tuple.0;
    let mut event_pump = setup_tuple.1;
    // Defines player and direction
    // TODO: potential change direction definitions
    // as the current implementation creates snake like movement
    let mut player     = Player::new(100, 100, 100, 100, 5);
    // FILE PATH NOT WORKING, PRODUCING UNRECOGNIZED AUDIO FORMAT
    let mut cool_music = AudioSource::new(500, 500, 50, 50, "/home/mitchell/Spacial-Sound/src/audio/mp3/Cellular.mp3");
    let mut direction  = Direction::NULL;
    // Game loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // Quit application with close key of ESC
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                // Up player movement
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    direction = Direction::N;
                },
                // Right player movement
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    direction = Direction::E;
                },
                // Down player movement
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    direction = Direction::S;
                },
                // Left player movement
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    direction = Direction::W;
                },
                // Allows code to be ran for all
                // events
                _ => {}
            }
        }
        update(&mut player, &direction);
        render(&player, &mut canvas);
    }
}

fn sdl_setup() -> (Canvas<Window>, EventPump) {
    // Initalize SDL
    let sdl_context = sdl2::init().unwrap();
    // Initalize SDL video
    let video_subsystem = sdl_context.video().unwrap();
    // Set up SDL window, centered to the screen
    let window = video_subsystem.window("Spacial Sound", globals::SCREEN_WIDTH, globals::SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    // Creates canvas with vsync    
    let canvas = window.into_canvas()
        .present_vsync()    
        .build()
        .unwrap();
    // Event pump used for detecting user input
    let event_pump = sdl_context.event_pump()
        .unwrap();

    (canvas, event_pump)
}

fn update(player: &mut Player, direction: &Direction) {
    player.update(direction);
}

fn render(player: &Player, canvas: &mut Canvas<Window>) {
    // Renders white background for window
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    player.render(canvas);
    // Shows rendered data to the screen
    canvas.present();
}
