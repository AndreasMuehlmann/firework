extern crate sdl2;
use sdl2::video::Window;
use sdl2::video::WindowBuilder;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::event::EventPollIterator;
use sdl2::EventPump;


pub struct SDL2Wrapper {
    canvas: Canvas<Window>,
    background_color: Color,
    event_pump: EventPump,
}

impl SDL2Wrapper {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window_builder = WindowBuilder::new(&video_subsystem, "firework", 1600, 1200);
        window_builder.position_centered();
        window_builder.maximized();
        window_builder.opengl();
        window_builder.resizable();

        let window = window_builder.build().unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let background_color = Color::RGB(0, 0, 0);
        
        return Self {
            canvas,
            background_color, 
            event_pump,
        }
    }

    pub fn fill_rect_with_color(&mut self, x_pos: i32, y_pos: i32, x_size: u32, y_size: u32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(x_pos, y_pos, x_size, y_size)).unwrap();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> EventPollIterator {
        return self.event_pump.poll_iter();
    }

    pub fn get_window_size(&mut self) -> (u32, u32) {
        return self.canvas.output_size().unwrap();
    }
}
