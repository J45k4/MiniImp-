use sdl2::{VideoSubsystem, Sdl, EventPump, event::Event, keyboard::Keycode, render::Canvas, rect::Point, pixels::Color};

struct Line {
    x: u32,
    y: u32,
    x2: u32,
    y2: u32,
}

pub struct Window {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    canvas: Canvas<sdl2::video::Window>,
    lines: Vec<Line>,
}

impl Window {
    pub fn new(name: &str) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem.window(name, 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Window { 
            sdl: sdl_context,
            video_subsystem: video_subsystem,
            event_pump: event_pump,
            canvas: canvas,
            lines: vec![],
        }
    }

    pub fn draw_line(&mut self, x: u32, y: u32, x2: u32, y2: u32) {
        self.lines.push(Line{
            x: x,
            y: y,
            x2: x2,
            y2: y2,
        });
    }

    pub fn draw_rect(&mut self) {

    }

    pub fn draw_circle(&mut self) {

    }

    pub fn work(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                _ => {}
            }
        }

        for line in self.lines.iter_mut() {
            self.canvas.set_draw_color(Color::RGB(255, 255, 0));
            self.canvas.draw_line(
                Point::new(line.x as i32, line.y as i32), 
                Point::new(line.x2 as i32, line.y2 as i32)
            ).unwrap();
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        self.canvas.present();

        true
    }
}