use fltk::{app, prelude::*, window::Window};

pub struct App;
impl App {
    pub fn runApp(&self) {
        let app = app::App::default();
        let mut window = Window::new(100,100,400,300, "Snake");
        window.end();
        window.show();
        app.run().unwrap();    
    }
}
