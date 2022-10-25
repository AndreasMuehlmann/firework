use std::thread;
use std::time::{Duration, SystemTime};
use std::thread::sleep;


extern crate sdl2;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


mod moving_object;
use moving_object::MovingObject;


const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 1200;
const TICKS_PER_SECOND: u32 = 25;
const SKIPPED_MILISECS_PER_TICK: f32 = (1000 / TICKS_PER_SECOND) as f32;
const MAX_FRAMESKIP: u32 = 5;
const MAX_FPS: f32 = 144.0;


struct FireworkAnimation {
    moving_objects: Vec<MovingObject>,
    particles: Vec<MovingObject>,
}


impl FireworkAnimation {
    pub fn new() -> Self {
        return Self {
            moving_objects: Vec::new(),
            particles: Vec::new(),
        }
    }

    fn tick(&mut self) {
    }

    fn display(&self) {

    }
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("firework", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut firework_animation = FireworkAnimation::new();

    let mut accumulator: f32 = 0.0;
    let mut loops: u32;
    let mut time_last_update = SystemTime::now();
    let mut delta_time_frames: f32 = 1.0 / MAX_FPS;

    'running: loop {
        let now = SystemTime::now();
        let delta_time = now.duration_since(time_last_update).unwrap();
        time_last_update += delta_time;
        accumulator += delta_time.as_nanos() as f32 / 1000.0;
        delta_time_frames += delta_time.as_nanos() as f32 / 1000.0; 

        loops = 0;
        while accumulator > SKIPPED_MILISECS_PER_TICK as f32 && loops < MAX_FRAMESKIP {
            firework_animation.tick();

            accumulator = accumulator - SKIPPED_MILISECS_PER_TICK;
            loops += 1;
        }

        if delta_time_frames / 1000.0 >= 1.0 / MAX_FPS - (1.0 / MAX_FPS) / 8.0 {
            firework_animation.display();
            delta_time_frames = 0.0;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },

                _ => {}
            }
        }
    }
}
