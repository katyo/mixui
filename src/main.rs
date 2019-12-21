mod platform;
mod sgl;

use std::path::PathBuf;

use platform::{Key, Button, Phase, EventHandler, Platform, ViewConfig};

use sgl::{GL, HasContext, demo::Demo};
use glow::{Context};

pub struct Application<G: HasContext> {
    text: String,
    demo: Option<Demo<G>>,
}

impl<G: HasContext> Application<G> {
    pub fn new() -> Self {
        Self { text: String::from("Text"), demo: None }
    }
}

impl<G: HasContext> EventHandler for Application<G> {
    type Context = G;

    fn input(&mut self, chr: char) {
        println!("input: {} #{}", chr, chr as u32);
        if chr as u32 >= 0x20 {
            self.text.push(chr);
        }
    }

    fn key(&mut self, key: Key, state: bool) {
        println!("key {}: {:?}", if state { "pressed" } else { "released" }, key);
        if state {
            match key {
                Key::Back => { self.text.pop(); },
                Key::Return => { self.text.push('\n'); },
                _ => (),
            }
        }
    }

    fn pointer(&mut self, x: f32, y: f32, _dot_ratio: f32) {
        //println!("pointer: ({}, {})", x, y);
    }

    fn button(&mut self, btn: Button, state: bool) {
        println!("button {}: {:?}", if state { "pressed" } else { "released" }, btn);
    }

    fn scroll(&mut self, dx: f32, dy: f32, pix: bool) {
        println!("scroll: ({}, {}) {}", dx, dy, if pix { "pixelwise" } else { "linear" });
    }

    fn touch(&mut self, x: f32, y: f32, phase: Phase) {
        println!("touch {:?}: ({}, {})", phase, x, y);
    }

    fn hover(&mut self, state: bool) {
        println!("hover: {}", state);
    }

    fn focus(&mut self, state: bool) {
        println!("focus: {}", state);
    }

    fn file_over(&mut self, path: PathBuf) {
        println!("file over: {:?}", path);
    }

    fn file_out(&mut self) {
        println!("file out");
    }

    fn file_drop(&mut self, path: PathBuf) {
        println!("file drop: {:?}", path);
    }

    fn reconf(&mut self, ViewConfig { color_bits, alpha_bits, stencil_bits, num_samples, srgb_mode, view_width, view_height, dot_ratio, .. }: ViewConfig, gl: &G) {
        println!("resize: {}x{} dot: {}", view_width, view_height, dot_ratio);

        if self.demo.is_none() {
            self.demo = Some(Demo::new(gl).expect("Unable to init demo"));
        }
    }

    fn redraw(&mut self, gl: &G) {
        //println!("redraw");
        if let Some(demo) = &self.demo {
            demo.render(gl);
        }
    }

    fn destroy(&mut self) {
        println!("Good luck...");
    }

    fn suspend(&mut self) {
        println!("Suspended");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }
}

fn main() {
    let platform = Platform::new();
    let application = Application::<Context>::new();

    platform.run(application);
}
