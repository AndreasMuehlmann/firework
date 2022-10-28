extern crate sdl2;
use sdl2::pixels::Color;


pub struct MovingObject {
    pub angle: f32,
    pub angular_vel: f32,
    pub pos: [f32; 2],     
    pub vel: [f32; 2],
    pub acc: [f32; 2],
    pub color: Color,
    pub ticks_to_explosion: u32, 
}

impl MovingObject {
    pub fn new(angle: f32, angular_vel: f32, pos: [f32; 2], vel: [f32; 2], acc: [f32; 2], color: Color, ticks_to_explosion: u32) -> Self {
        return Self {
            angle, 
            angular_vel,
            pos,
            vel,
            acc,
            color,
            ticks_to_explosion, 
        }
    }

    pub fn move_object(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
    }
}
