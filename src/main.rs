extern crate sdl2;

#[path = "./player/player.rs"]
mod player;
#[path = "./player/direction.rs"]
mod direction;
#[path = "./audio/audio_source.rs"]
mod audio_source;
// Global varaibles
mod globals;
mod collision;

use sdl2::mixer::{InitFlag, DEFAULT_CHANNELS, AUDIO_S16LSB};
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use player::Player;
use direction::Direction;
use audio_source::AudioSource;

fn main() {
    // Sets up SDL and get required variables for
    // operations
    let setup_tuple    = sdl_setup();
    let mut canvas     = setup_tuple.0;
    let mut event_pump = setup_tuple.1;
    // 44kHz
    let frequency = 44_100;
    // Signed 16 bit samples
    let format = AUDIO_S16LSB;
    // Stereo
    let channels = DEFAULT_CHANNELS;
    // 1Mb
    let chunck_size = 1_024;
    // Opens audio channels
    sdl2::mixer::open_audio(frequency, format, channels, chunck_size).unwrap();
    // Defines player and direction
    // TODO: potential change direction definitions
    // as the current implementation creates snake like movement
    let mut player     = Player::new(100, 100, 100, 100, 5);
    let mut cool_music = Vec::new();
    cool_music.push(AudioSource::new(500, 500, 50, 50, "/home/mitchell/Spacial-Sound/src/audio/flac/waiting_so_long.flac", 0, 100, 500));
    cool_music.push(AudioSource::new(800, 100, 25, 25, "/home/mitchell/Spacial-Sound/src/audio/flac/gettin_freaky.flac", 1, 100, 500));
    let mut direction  = Direction::NULL;
    // Play music
    cool_music[0].play();
    cool_music[1].play();
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

        update(&mut player, &cool_music, &direction);
        render(&player, &cool_music, &mut canvas);
    }
}

fn sdl_setup() -> (Canvas<Window>, EventPump) {
    // Initalize SDL
    let sdl_context = sdl2::init().unwrap();
    // Initalize SDL audio
    sdl_context.audio().unwrap();
    // Initalize SDL video
    let video_subsystem = sdl_context.video().unwrap();
    // 4 channel mixing, simultaneously
    let channel_amount = 4;
    // Initalize SDL mixer
    sdl2::mixer::init(InitFlag::FLAC).unwrap();
    // Allocated channels
    sdl2::mixer::allocate_channels(channel_amount);
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

fn update(player: &mut Player, cool_music: &Vec<AudioSource>, direction: &Direction) {
    if !collision_check(&player, cool_music) {
        player.update(direction);
    }

    for music in cool_music {
        music.update(player);
    }
}

fn render(player: &Player, cool_music: &Vec<AudioSource>, canvas: &mut Canvas<Window>) {
    // Renders white background for window
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    player.render(canvas);

    for music in cool_music {
        music.render(canvas);
    }
    // Shows rendered data to the screen
    canvas.present();
}

fn collision_check(player: &Player, cool_music: &Vec<AudioSource>) -> bool {
    // Screen boarder check
    let mut collided = collision::screen_boarder(&player.collider());
    if collided {return collided}
    // Checks all audio sources for collision
    for music in cool_music {
        collided = collision::axis_aligned(&player.collider(), &music.collider());
        if collided {return collided}
    }

    return collided;
}
