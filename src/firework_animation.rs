use std::process;
use rand::{thread_rng, Rng};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::SDL2Wrapper;

mod rocket;
use rocket::Rocket;


pub struct FireworkAnimation {
    sdl2_wrapper: SDL2Wrapper,
    rockets: Vec<Rocket>,
    rocket_spawn_rate: f64,
    // particles: Vec<MovingObject>,
}


impl FireworkAnimation {
    pub fn new(sdl2_wrapper: SDL2Wrapper) -> Self {
        return Self {
            sdl2_wrapper,
            rockets: Vec::new(),
            rocket_spawn_rate: 0.2,
            // particles: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        let window_size = self.sdl2_wrapper.get_window_size();
        let bounds = [
            -1.0 * window_size.0 as f32 / 10.0 , window_size.0 as f32 + window_size.0 as f32 / 10.0, 
            -1.0 * window_size.1 as f32 / 10.0 , window_size.1 as f32 + window_size.1 as f32 / 10.0
        ];
        let mut rng = thread_rng();

        if rng.gen::<f64>() < self.rocket_spawn_rate {
            let rocket = self.create_rocket(bounds);
            self.rockets.push(rocket);
        }

        let mut index: usize = 0;
        let mut to_explode: Vec<usize> = Vec::new();
        for rocket in &mut self.rockets {
            rocket.move_rocket();
            rocket.substract_tick();
            if !FireworkAnimation::is_in_bounds(rocket.pos, bounds) || rocket.ticks_to_explosion <= 0 {
                to_explode.push(index);
            }
            index += 0;
        }
        for to_explode_index in to_explode {
            //self.explosion(self.rocketsto_explode_index)
            self.rockets.remove(to_explode_index);
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

    fn create_rocket(&self, pos_bounds: [f32; 4]) -> Rocket {
        let mut rng = thread_rng();
        return Rocket::new(
            [10, 20],
            0.0,
            0.0,
            [pos_bounds[0] + rng.gen::<f64>() as f32 * (pos_bounds[1] - pos_bounds[0]), pos_bounds[3]],
            [(rng.gen::<f64>() - 0.5) as f32, (-rng.gen::<f64>() * 5.0) as f32],
            [((rng.gen::<f64>() - 0.5) * 0.2) as f32, (-0.5 - rng.gen::<f64>() * 0.5) as f32],
            Color::RGB(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()),
            (10.0 + rng.gen::<f64>() * 65.0) as u32,
            );
    }
    
    fn is_in_bounds(pos: [f32; 2], bounds: [f32; 4]) -> bool {
        return bounds[0] <= pos[0] && pos[0] <= bounds[1]
            && bounds[2] <= pos[1] && pos[1] <= bounds[3];
    }

    fn explosion(&self, rocket: Rocket) {
         
    }

    pub fn display(&mut self) {
        self.sdl2_wrapper.clear();
        for rocket in &self.rockets {
            self.sdl2_wrapper.fill_rect_with_color(rocket.pos[0] as i32, rocket.pos[1] as i32,
                                                   rocket.size[0], rocket.size[1], rocket.color);
        }
        self.sdl2_wrapper.present();
    }
}
