extern crate sdl2;

#[path = "./player/player.rs"]
mod player;
#[path = "./player/direction.rs"]
mod direction;
#[path = "./audio/audio_source.rs"]
mod audio_source;
// Global varaibles
mod globals;
#[path = "./collision/collision.rs"]
mod collision;
#[path = "./collision/collision_map.rs"]
mod collision_map;

use collision_map::CollisionMap;
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
    let mut player          = Player::new(100, 100, 100, 100, 5);
    let mut player_previous = player.collider();
    let mut cool_music      = Vec::new();
    cool_music.push(AudioSource::new(0, 500, 500, 150, 50, "/home/mitchell/Spacial-Sound/src/audio/flac/waiting_so_long.flac", 0, 100, 500, 100));
    cool_music.push(AudioSource::new(1, 800, 120, 25, 25, "/home/mitchell/Spacial-Sound/src/audio/flac/gettin_freaky.flac", 1, 100, 500, 100));
    let mut collision_map = CollisionMap::new();
    collision_map.set_direction(cool_music[0].id(), Direction::NULL);
    collision_map.set_direction(cool_music[1].id(), Direction::NULL);
    let mut direction  = Direction::NULL;
    let mut last_frame_collision = false;
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
                Event::KeyUp {keycode: Some(Keycode::W), ..} => {
                    if direction == Direction::N {
                        direction = Direction::NULL;
                    }
                },
                // Right player movement
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    direction = Direction::E;
                },
                Event::KeyUp {keycode: Some(Keycode::D), ..} => {
                    if direction == Direction::E {
                        direction = Direction::NULL;
                    }
                },
                // Down player movement
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    direction = Direction::S;
                },
                Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                    if direction == Direction::S {
                        direction = Direction::NULL;
                    }
                },
                // Left player movement
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    direction = Direction::W;
                },
                Event::KeyUp {keycode: Some(Keycode::A), ..} => {
                    if direction == Direction::W {
                        direction = Direction::NULL;
                    }
                },
                // All other cases do nothing
                _ => {}
            }
        }

        update(&mut player, &mut player_previous, &cool_music, &mut collision_map, &direction, &mut last_frame_collision);
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

fn update(player: &mut Player, player_previous: &mut Rect, cool_music: &Vec<AudioSource>, collision_map: &mut CollisionMap, direction: &Direction, last_frame_collision: &mut bool) {
    // Gets collision direction, If collision did not occur Direction::NULL is returned
    collision_check(&player, cool_music, collision_map);
    let screen_collision_tuple = screen_collision_check(&player);
    
    // First, checks for screen bound collision
    if (screen_collision_tuple == (Direction::NULL, Direction::NULL) 
    // Checks for tuple collision with screen bounds
    || screen_collision_tuple.0 != *direction && screen_collision_tuple.1 != *direction)
    // Check for collision with other world rects
    && (!collision_map.check_for_direction(direction)) {
        *player_previous = player.collider();
        player.update(direction);
    }
    // First, checks for screen bound collision 
    else if screen_collision_tuple == (Direction::NULL, Direction::NULL) && 
    // Checks for movement and If the last frame involved a collision
    // The last frame check reduces overall computation
    *direction != Direction::NULL && *last_frame_collision {
        let new_collider = player_previous;
        // Overlap check that saves new collider value in new_collider
        if overlap_check(new_collider, direction, cool_music, collision_map) {
            player.set_collider(*new_collider);
        }
        // Reset last frame collision
        *last_frame_collision = false
    } else {
        // Set last frame as collided
        *last_frame_collision = true;
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

// Redundent wrapping, however was added in case of additional checks
fn screen_collision_check(player: &Player) -> (Direction, Direction) {
    return collision::screen_boarder(&player.collider());
}

fn collision_check(player: &Player, cool_music: &Vec<AudioSource>, collision_map: &mut CollisionMap) {
    // Checks all audio sources for collision
    for music in cool_music {
        let collided = collision::axis_aligned(&player.collider(), &music.collider());
        // Sets collision direction in collision_map If collision was sucessful
        if collided {
            collision_map.set_direction(music.id(), collision::axis_aligned_direction(&player.collider(), &music.collider()));
        }
        // Sets collision direction to Direction::NULL If collision did not occur
        else if *collision_map.get_direction(music.id()).unwrap() != Direction::NULL {
            collision_map.set_direction(music.id(), Direction::NULL);
        }
    }
}

fn overlap_check(new_collider: &mut Rect, direction: &Direction, cool_music: &Vec<AudioSource>, collision_map: &CollisionMap) -> bool {
    // Gets the first collider from music array using our movement direction

    // Shouldent create issues since our player is a square, If player
    // shape changes (I.e. circle) this may need to be changed
    let collision_id = collision_map.get_first_id(direction);
    let collider     = &cool_music[*collision_id as usize].collider();

    return collision::axis_aligned_continous(new_collider, direction, collider);
}
