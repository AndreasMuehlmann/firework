use std::time::SystemTime;
use std::{thread, time};

mod firework_animation;
use firework_animation::FireworkAnimation;

mod sdl2_wrapper;
use sdl2_wrapper::SDL2Wrapper;


const TICKS_PER_SECOND: u32 = 100;
const NANO_SECS_BETWEEN_TICKS: f32 = (10_u32.pow(9) / TICKS_PER_SECOND) as f32;
const FRAMES_PER_SECOND: f32 = 144.0;
const NANO_SECS_BETWEEN_FRAMES: f32 = 10_u32.pow(9) as f32 / FRAMES_PER_SECOND;
const MAX_FRAMESKIP: u32 = TICKS_PER_SECOND;
const VALUE_DIVIDING_SLEEP_TIME: f32 = 10.0;


pub fn main() {
    let sdl2_wrapper = SDL2Wrapper::new();
    let mut firework_animation = FireworkAnimation::new(sdl2_wrapper);

    let mut time_last_update = SystemTime::now();
    let mut accumulator_ticks: f32 = 0.0;
    let mut accumulator_frames: f32 = 1.0 / NANO_SECS_BETWEEN_FRAMES;
    let mut loops: u32;

    loop {
        let now = SystemTime::now();
        let delta_time = now.duration_since(time_last_update).unwrap();
        time_last_update += delta_time;
        accumulator_ticks += delta_time.as_nanos() as f32;
        accumulator_frames += delta_time.as_nanos() as f32; 

        loops = 0;
        while accumulator_ticks > NANO_SECS_BETWEEN_TICKS as f32 && loops < MAX_FRAMESKIP {
            firework_animation.tick();

            accumulator_ticks -= NANO_SECS_BETWEEN_TICKS;
            loops += 1;
        }

        if accumulator_frames >= NANO_SECS_BETWEEN_FRAMES {
            firework_animation.display();
            accumulator_frames = 0.0;
        }

        if accumulator_ticks > NANO_SECS_BETWEEN_TICKS {
            continue
        }

        let nano_secs_to_sleep: f32;
        if NANO_SECS_BETWEEN_TICKS - accumulator_ticks < NANO_SECS_BETWEEN_FRAMES - accumulator_frames {
            nano_secs_to_sleep = (NANO_SECS_BETWEEN_TICKS - accumulator_ticks) / VALUE_DIVIDING_SLEEP_TIME;
        } else {
            nano_secs_to_sleep = (NANO_SECS_BETWEEN_FRAMES - accumulator_frames) / VALUE_DIVIDING_SLEEP_TIME;
        } 
        thread::sleep(time::Duration::from_nanos(nano_secs_to_sleep as u64));
    }
}
