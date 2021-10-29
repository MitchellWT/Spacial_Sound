extern crate sdl2;

#[path = "./player/player.rs"]
mod player;
#[path = "./player/direction.rs"]
mod direction;

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

fn main() {
    let setup_tuple = sdl_setup();
    let mut canvas = setup_tuple.0;
    let mut event_pump = setup_tuple.1;

    let mut player = Player::new(100, 100, 100, 100, 5);
    let mut direction = Direction::NULL;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    direction = Direction::N;
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    direction = Direction::E;
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    direction = Direction::S;
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    direction = Direction::W;
                },
                _ => {}
            }
        }
        update(&mut player, &direction);
        render(&player, &mut canvas);
    }
}

fn sdl_setup() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Spacial Sound", globals::SCREEN_WIDTH, globals::SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    
    let canvas = window.into_canvas()
        .present_vsync()    
        .build()
        .unwrap();

    let event_pump = sdl_context.event_pump()
        .unwrap();

    (canvas, event_pump)
}

fn update(player: &mut Player, direction: &Direction) {
    player.update(direction);
}

fn render(player: &Player, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    player.render(canvas);

    canvas.present();
}