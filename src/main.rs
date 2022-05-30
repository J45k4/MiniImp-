use std::time::Duration;

use compiler::compile;

mod parser;
mod compiler;
mod bytecode;
mod vm;
mod window;
use sdl2::{pixels::Color, keyboard::Keycode, event::Event, rect::Point};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.draw_line(Point::new(20, 20), Point::new(100, 100)).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // let file_content = std::fs::read_to_string("./example.mi").unwrap();

    // let ast = parser::parse_text(&file_content).unwrap();

    // println!("{:#?}", ast);

    // let mut vm = compile(ast);

    // println!("{:?}", vm);

    // vm.run();

    //println!("{:?}", vm);
}
