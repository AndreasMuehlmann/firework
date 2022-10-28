extern crate sdl2;
use sdl2::pixels::Color;


pub struct Particle {
    pub size: [u32; 2],
    pub pos: [f32; 2],     
    pub vel: [f32; 2],
    pub acc: [f32; 2],
    pub color: Color,
    pub opacity: u32, 
}


impl Particle {
    pub fn new(size: [u32; 2], pos: [f32; 2], vel: [f32; 2], acc: [f32; 2], color: Color, opacity: u32) -> Self {
        return Self {
            size,
            pos,
            vel,
            acc,
            color,
            opacity, 
        }
    }

    pub fn move_particle(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
    }

    pub fn substract_tick_opacity(&mut self) {
        if self.opacity < 5 {
            self.opacity = 0;
            return;
        }
        self.opacity -= 5;
    }
}
