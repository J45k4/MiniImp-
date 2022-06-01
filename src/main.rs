use std::time::{Duration, Instant};

use compiler::compile;

mod parser;
mod compiler;
mod bytecode;
mod vm;
mod window;
use sdl2::{pixels::Color, keyboard::Keycode, event::Event, rect::Point};
use window::Window;

fn main() {
    let file_content = std::fs::read_to_string("./example.mi").unwrap();
    let ast = parser::parse_text(&file_content).unwrap();

    let mut vm = compile(ast);

    println!("{:?}", vm);

    let mut win = Window::new("Big bop");

    let mut sleep: Option<(Instant, u32)> = None;

    loop {
        if !win.work() {
            break;
        }

        let sleeping = match sleep {
            Some((i, t)) => {
                if i.elapsed().as_millis() > t.into() {
                    false
                } else {
                    true
                }                
            },
            None => false
        };

        if !sleeping {
            match vm.work() {
                vm::VmRunResult::Continue => {},
                vm::VmRunResult::Stop => return,
                vm::VmRunResult::Actions(actions) => {
                    for a in actions {
                        match a {
                            vm::Action::Sleep(t) => {
                                sleep = Some((Instant::now(), t));
                            },
                            vm::Action::Line(x, y, x2, y2) => {
                                win.draw_line(x, y, x2, y2);
                            },
                            vm::Action::Rectangle(x, y, x2, y2) => {
                                win.draw_rect(x, y, x2, y2);
                            },
                            vm::Action::Circle(x, y, r) => {
                                win.draw_circle(x, y, r as i32);
                            },
                                                   
                        }
                    }
                },
            };
        }
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
