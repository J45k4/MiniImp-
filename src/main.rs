use std::{time::{Duration, Instant}, env};

use compiler::compile;

mod parser;
mod compiler;
mod bytecode;
mod vm;
mod window;
use log::LevelFilter;
use window::Window;

enum RunMode {
    Debug,
    Dis,
    Normal
}

fn has_arg(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn main() {
    let mut logger_builder = env_logger::Builder::new();
    logger_builder.format_timestamp(None);

    let args: Vec<String> = env::args().collect();

    let app = &args[1];

    let (path, mode) = match app.as_str() {
        "dis" => {
            (args[2].as_str(), RunMode::Dis)
        },
        "debug" => {
            (args[2].as_str(), RunMode::Debug)
        },
        s => {
            (s, RunMode::Normal)
        }
    };

    if has_arg(&args, "--verbose") {
        logger_builder.filter_level(LevelFilter::Debug);
    } else {
        logger_builder.filter_level(LevelFilter::Info);
    }

    logger_builder.init();

    log::info!("file path: {}", path);

    let file_content = std::fs::read_to_string(path).unwrap();
    let ast = parser::parse_text(&file_content).unwrap();
    let mut vm = compile(ast);

    if let RunMode::Dis = mode {
        vm.disassemble();
        return;
    }

    let mut win = Window::new("Big bob");

    let mut sleep: Option<(Instant, u32)> = None;

    loop {
        if !win.work() {
            break;
        }

        let sleeping = match sleep {
            Some((i, t)) => {
                if i.elapsed().as_millis() > t.into() {
                    sleep = None;

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
                            vm::Action::Line(x, y, x2, y2, color) => {
                                win.draw_line(x, y, x2, y2, color);
                            },
                            vm::Action::Rectangle(x, y, x2, y2, color) => {
                                win.draw_rect(x, y, x2, y2, color);
                            },
                            vm::Action::Circle(x, y, r, color) => {
                                win.draw_circle(x, y, r as i32, color);
                            },
                            vm::Action::Clear => {
                                win.clear();
                            }                     
                        }
                    }
                },
            };
        }
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
