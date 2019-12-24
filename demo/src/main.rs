use apl::{EventHandler, Platform, ViewConfig, AppConfig};
use sgl::{GL, HasContext, Context, demo::Demo};

pub struct Application<G: HasContext> {
    demo: Option<Demo<G>>,
}

impl<G: HasContext> Application<G> {
    pub fn new() -> Self {
        Self { demo: None }
    }
}

impl<G: HasContext> EventHandler for Application<G> {
    type Context = G;

    fn reconf(&mut self, ViewConfig { view_width, view_height, dot_ratio, .. }: ViewConfig, gl: &G) {
        println!("resize: {}x{} dot: {}", view_width, view_height, dot_ratio);

        if self.demo.is_none() {
            self.demo = Some(Demo::new(gl).expect("Unable to init demo"));
        }
    }

    fn redraw(&mut self, gl: &G) {
        //println!("redraw");
        unsafe { gl.clear(GL::COLOR_BUFFER_BIT); }

        if let Some(demo) = &self.demo {
            demo.render(gl);
        }
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let config = AppConfig {
        title: "GLES Demo".into(),
        icon: None,
    };

    let platform = Platform::new(config);

    let application = Application::<Context>::new();

    platform.run(application);
}
