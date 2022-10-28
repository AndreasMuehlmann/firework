extern crate sdl2;
use sdl2::pixels::Color;


pub struct Rocket {
    pub size: [u32; 2],
    pub pos: [f32; 2],     
    pub vel: [f32; 2],
    pub acc: [f32; 2],
    pub color: Color,
    pub ticks_to_explosion: u32, 
}


impl Rocket {
    pub fn new(size: [u32; 2], pos: [f32; 2], vel: [f32; 2], acc: [f32; 2], color: Color, ticks_to_explosion: u32) -> Self {
        return Self {
            size,
            pos,
            vel,
            acc,
            color,
            ticks_to_explosion, 
        }
    }

    pub fn move_rocket(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
    }

    pub fn substract_tick(&mut self) {
        if self.ticks_to_explosion < 1 {
            self.ticks_to_explosion = 0;
            return;
        }
        self.ticks_to_explosion -= 1;
    }
}
