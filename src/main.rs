extern crate sdl2;

mod audio;
mod player;
mod globals;
mod collision;

use audio::wall::Wall;
use audio::audio_source::AudioSource;
use collision::collision_map::CollisionMap;
use player::player::Player;
use player::entity::Entity;
use player::direction::Direction;
use sdl2::mixer::{InitFlag, DEFAULT_CHANNELS, AUDIO_S16LSB};
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

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
    //let mut player_previous = player.collider();
    let mut entity_vec: Vec<Box<dyn Entity>> = Vec::new();
    let mut collision_map = CollisionMap::new();
    let mut player = Player::new(0, 100, 100, 100, 100, 5);
    let mut player_previous = player.collider();
    entity_vec.push(Box::new(AudioSource::new(0, 500, 500, 150, 50, "/home/mitchell/Spacial-Sound/src/audio/flac/waiting_so_long.flac", 0, 100, 500, 100)));
    entity_vec.push(Box::new(AudioSource::new(1, 800, 120, 25, 25, "/home/mitchell/Spacial-Sound/src/audio/flac/gettin_freaky.flac", 1, 100, 500, 100)));
    entity_vec.push(Box::new(Wall::new(2, 300, 500, 100, 50)));
    collision_map.set_direction(entity_vec[0].id(), Direction::NULL);
    collision_map.set_direction(entity_vec[1].id(), Direction::NULL);
    collision_map.set_direction(entity_vec[2].id(), Direction::NULL);
    let mut direction  = Direction::NULL;
    let mut last_frame_collision = false;
    // Play music
    entity_vec[0].as_any().downcast_ref::<AudioSource>().unwrap().play();
    entity_vec[1].as_any().downcast_ref::<AudioSource>().unwrap().play();
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

        update(&mut player, &mut entity_vec, &mut player_previous, &mut collision_map, &direction, &mut last_frame_collision);
        render(&mut player, &mut entity_vec, &mut canvas);
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

fn update(player: &mut Player, entity_vec: &mut Vec<Box<dyn Entity>>, player_previous: &mut Rect, collision_map: &mut CollisionMap, direction: &Direction, last_frame_collision: &mut bool) {
    // Gets collision direction, If collision did not occur Direction::NULL is returned
    collision_check(player, entity_vec, collision_map);
    let screen_collision_tuple = screen_collision_check(player);
    
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
        if overlap_check(new_collider, direction, entity_vec, collision_map) {
            player.set_collider(*new_collider);
        }
        // Reset last frame collision
        *last_frame_collision = false
    } else {
        // Set last frame as collided
        *last_frame_collision = true;
    }

    let mut audio_source_vec = Vec::new();
    let mut wall_vec = Vec::new();

    for entity in entity_vec {
        match entity.as_any().downcast_ref::<AudioSource>() {
            Some(audio_source) => audio_source_vec.push(audio_source),
            None => {}
        }
        match entity.as_any().downcast_ref::<Wall>() {
            Some(wall) => wall_vec.push(wall),
            None => {}
        }
    }

    for audio_source in audio_source_vec {
        let mut interference_amount = 0;
        for wall in &wall_vec {
            interference_amount += wall.get_interference_amount(player.collider().center(), audio_source.collider().center());
        }

        audio_source.update(&player, &interference_amount);
    }
}

fn render(player: &mut Player, entity_vec: &Vec<Box<dyn Entity>>, canvas: &mut Canvas<Window>) {
    // Renders white background for window
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    player.render(canvas);

    for entity in entity_vec {
        entity.render(canvas);
    }

    // Shows rendered data to the screen
    canvas.present();
}

// Redundent wrapping, however was added in case of additional checks
fn screen_collision_check(player: &Player) -> (Direction, Direction) {
    return collision::screen_boarder(&player.collider());
}

fn collision_check(player: &Player, entity_vec: &Vec<Box<dyn Entity>>, collision_map: &mut CollisionMap) {
    // Checks all audio sources for collision
    for entity in entity_vec {
        let collided = collision::axis_aligned(&player.collider(), &entity.collider());
        // Sets collision direction in collision_map If collision was sucessful
        if collided {
            if entity.collider().width() == entity.collider().height() {
                collision_map.set_direction(entity.id(), collision::axis_aligned_direction(&player.collider(), &entity.collider()));
            } else {
                collision_map.set_direction(entity.id(), collision::line_to_line_direction(&player.collider(), &entity.collider()));    
            }
        }
        // Sets collision direction to Direction::NULL If collision did not occur
        else if *collision_map.get_direction(entity.id()).unwrap() != Direction::NULL {
            collision_map.set_direction(entity.id(), Direction::NULL);
        }
    }
}

fn overlap_check(new_collider: &mut Rect, direction: &Direction, entity_vec: &Vec<Box<dyn Entity>>, collision_map: &CollisionMap) -> bool {
    // Gets the first collider from music array using our movement direction

    // Shouldent create issues since our player is a square, If player
    // shape changes (I.e. circle) this may need to be changed
    let collision_id = collision_map.get_first_id(direction);
    let collider     = &entity_vec[*collision_id as usize].collider();

    return collision::axis_aligned_continous(new_collider, direction, collider);
}
