pub struct MovingObject {
    pub angle: f32,
    pub pos: [f32; 2],     
    pub vel: [f32; 2],
    pub acc: [f32; 2],
}

impl MovingObject {
    pub fn new(angle: f32, pos: [f32; 2], vel: [f32; 2], acc: [f32; 2]) -> Self {
        return Self {
            angle, 
            pos,
            vel,
            acc,
        }
    }
    pub fn move_object(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
    }
}
