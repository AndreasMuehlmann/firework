pub struct MovingObject {
    angle: f32,
    pos: f32,     
    vel: f32,
    acc: f32,
}

impl MovingObject {
    pub fn new(angle: f32, pos: f32, vel: f32, acc: f32) -> Self {
        return Self {
            angle, 
            pos,
            vel,
            acc,
        }
    }
}
