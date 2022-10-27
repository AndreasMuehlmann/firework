use std::process;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::SDL2Wrapper;

mod moving_object;
use moving_object::MovingObject;


pub struct FireworkAnimation {
    sdl2_wrapper: SDL2Wrapper,
    rockets: Vec<MovingObject>,
    // particles: Vec<MovingObject>,
}


impl FireworkAnimation {
    pub fn new(sdl2_wrapper: SDL2Wrapper) -> Self {
        return Self {
            sdl2_wrapper,
            rockets: Vec::new(),
            // particles: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        let window_size = self.sdl2_wrapper.get_window_size();
        let rocket = MovingObject::new(0.0, [(window_size.0 / 2) as f32, window_size.1 as f32], [-0.0, -1.0], [-0.1, 0.001]);
        self.rockets.push(rocket);

        for rocket in &mut self.rockets {
            rocket.move_object();
        }
        for event in self.sdl2_wrapper.get_events() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        process::exit(0);
                    },

                _ => {}
            }
        }
    }

    pub fn display(&mut self) {
        self.sdl2_wrapper.set_draw_color(Color::RGB(250, 10, 10));
        for rocket in &self.rockets {
            self.sdl2_wrapper.fill_rect(rocket.pos[0] as i32, rocket.pos[1] as i32, 20, 40);
        }
        self.sdl2_wrapper.present();
    }
}
