// use beryllium::{init::Sdl, window::WindowFlags, gl_window::GlWindow};
// use zstring::zstr;

// pub struct Window {
//     win: GlWindow,
// }

// impl Window {
//     pub fn new(sdl: Sdl) -> Window {
//         let win = sdl
//         .create_gl_window(
//             zstr!("Hello Window"),
//             Some((100, 100)),
//             (400, 400),
//             WindowFlags::OPENGL,
//             )
//             .expect("couldn't make a window and context");


//         win.

//         unsafe {
//             //ogl33::load_gl_with(|f_name| win.get_proc_address(f_name));
//         }

//         Window {
//             win: win
//         }
//     }

//     pub fn draw_line(&mut self) {
//         // ogl33::gl
//     }

//     pub fn draw_rect(&mut self) {

//     }

//     pub fn draw_circle(&mut self) {

//     }
// }

// pub struct WindowManager {
//     sdl: Sdl
// }

// impl WindowManager {
//     pub fn new(sdl: Sdl) -> WindowManager {
//         WindowManager { 
//             sdl: sdl
//         }
//     }

//     pub fn create_win(&self, name: &str) -> Window {
//         Window::new(self.sdl.clone())
//     }
// }