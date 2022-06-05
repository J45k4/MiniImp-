use sdl2::{VideoSubsystem, Sdl, EventPump, event::Event, keyboard::Keycode, render::Canvas, rect::{Point, Rect}, pixels::Color};

fn convert_color(color: &str) -> Color {
    match color {
        "black" => Color::RGB(0, 0, 0),
        "white" => Color::RGB(255, 255, 255),
        "red" => Color::RGB(255, 0, 0),
        "green" => Color::RGB(0, 255, 0),
        "blue" => Color::RGB(0, 0, 255),
        "yellow" => Color::RGB(255, 255, 0),
        "magenta" => Color::RGB(255, 0, 255),
        "cyan" => Color::RGB(0, 255, 255),
        _ => panic!("invalid color")
    }
}

pub struct Window {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    canvas: Canvas<sdl2::video::Window>,
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

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.clear();

        Window { 
            sdl: sdl_context,
            video_subsystem: video_subsystem,
            event_pump: event_pump,
            canvas: canvas
        }
    }

    pub fn draw_line(&mut self, x: u32, y: u32, x2: u32, y2: u32, color: String) {
        self.canvas.set_draw_color(convert_color(color.as_str()));
        self.canvas.draw_line(
            Point::new(x as i32, y as i32), 
            Point::new(x2 as i32, y2 as i32)
        ).unwrap();
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: String) {
        self.canvas.set_draw_color(convert_color(color.as_str()));
        self.canvas.fill_rect(Rect::new(x as i32, y as i32, width, height)).unwrap();
    }

    pub fn draw_circle(&mut self, x: u32, y: u32, r: i32, color: String) {
        self.canvas.set_draw_color(convert_color(color.as_str()));

        let x = x as i32;
        let y = y as i32;

        let mut offsetx = 0;
        let mut offsety = r;
        let mut d = r - 1;

        while offsety >= offsetx {
            self.canvas.draw_line(
                Point::new(x - offsety, y + offsetx), 
                Point::new(x + offsety, y + offsetx)).unwrap();

            self.canvas.draw_line(
                Point::new(x - offsetx, y + offsety), 
                Point::new(x + offsetx, y + offsety)).unwrap();
            
            self.canvas.draw_line(
                Point::new(x - offsetx, y - offsety), 
                Point::new(x + offsetx, y - offsety)).unwrap();

            self.canvas.draw_line(
                Point::new(x - offsety, y - offsetx), 
                Point::new(x + offsety, y - offsetx)).unwrap();

            if d >= 2 * offsetx {
                d -= 2 * offsetx + 1;
                offsetx += 1;
            } else if d < 2 * (r - offsety) {
                d += 2 * offsety - 1;
                offsety -= 1;
            } else {
                d += 2 * (offsety - offsetx - 1);
                offsety -= 1;
                offsetx += 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
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

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.canvas.present();

        true
    }
}