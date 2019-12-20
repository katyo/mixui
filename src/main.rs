mod platform;

use std::path::PathBuf;

use platform::{Key, Button, Phase, EventHandler, Platform, ViewConfig};

use skia_safe::{gpu, Surface, SurfaceProps, SurfacePropsFlags, Color, Paint, paint::Style, ImageInfo, Budgeted, ColorSpace, ColorType};

use gl;

use std::ffi::c_void;

pub struct Application {
    context: gpu::Context,
    surface: Option<Surface>,
    text: String,
}

impl Application {
    pub fn new<F: FnMut(&str)-> *const c_void>(get_proc: F) -> Self {
        let interface = gpu::gl::Interface::new_load_with(get_proc)
            .expect("Invalid GPU interface");

        //let interface = None;
        let context = gpu::Context::new_gl(interface)
            .expect("Unable to create skia GPU context");

        Self { context, surface: None, text: String::default(), }
    }

    pub fn draw(&mut self) {
        if let Some(surface) = &mut self.surface {
            let canvas = surface.canvas();

            canvas.clear(Color::WHITE);

            let mut p = Paint::default();
            p.set_color(Color::RED);
            p.set_anti_alias(true);
            p.set_style(Style::Stroke);
            p.set_stroke_width(10.0);

            canvas.draw_line((20, 20), (100, 100), &p);
        }
    }
}

impl EventHandler for Application {
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

    fn reconf(&mut self, ViewConfig { color_bits, alpha_bits, stencil_bits, num_samples, srgb_mode, view_width, view_height, dot_ratio, .. }: ViewConfig) {
        println!("resize: {}x{} dot: {}", view_width, view_height, dot_ratio);

        if self.surface.is_some() {
            self.surface = None;
        }

        /*
        let image_info = ImageInfo::new_n32_premul(
            (view_width as i32, view_height as i32),
            if srgb_mode { Some(ColorSpace::new_srgb()) } else { None }
        );

        let surface = Surface::new_render_target(
            &mut self.context,
            Budgeted::YES,
            &image_info,
            None, // sample count
            gpu::SurfaceOrigin::BottomLeft, // surface origin
            None, // &surface_props,
            None, // should create with mips
        ).expect("Unable to create surface");
         */

        let format = if srgb_mode {
            match (color_bits, alpha_bits) {
                (24, 8) => gpu::gl::Format::SRGB8_ALPHA8,
                _ => gpu::gl::Format::Unknown,
            }
        } else {
            match (color_bits, alpha_bits) {
                (24, 8) => gpu::gl::Format::RGBA8,
                (12, 4) => gpu::gl::Format::RGBA4,
                (30, 2) => gpu::gl::Format::RGB10_A2,
                (16, 0) => gpu::gl::Format::RGB565,
                (8, 0) => gpu::gl::Format::R8,
                (0, 8) => gpu::gl::Format::ALPHA8,
                _ => gpu::gl::Format::Unknown,
            }
        };

        let mut fboid = 0i32;

        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid); }

        let framebuffer_info = gpu::gl::FramebufferInfo {
            fboid: fboid as u32,
            format: format as u32,
        };

        let backend_render_target = gpu::BackendRenderTarget::new_gl(
            (view_width as i32, view_height as i32),
            num_samples as usize,
            stencil_bits as usize,
            framebuffer_info,
        );

        if !backend_render_target.is_valid() {
            panic!("Unable to create valid backend render target");
        }

        let surface_props = SurfaceProps::default();

        let color_type = match (color_bits, alpha_bits) {
            (24, 8) => ColorType::RGBA8888,
            (12, 4) => ColorType::ARGB4444,
            (30, 2) => ColorType::RGBA1010102,
            (16, 0) => ColorType::RGB565,
            (32, 0) => ColorType::RGB888x,
            (8, 0) => ColorType::Gray8,
            (0, 8) => ColorType::Alpha8,
            _ => panic!("Unsupported color type for color_bits: {} and alpha bits: {}", color_bits, alpha_bits),
        };

        let color_space = if srgb_mode {
            println!("Use sRGB color space");
            Some(ColorSpace::new_srgb())
        } else {
            None
        };

        let surface = Surface::from_backend_render_target(
            &mut self.context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft, // surface origin
            color_type,
            color_space,
            None, //Some(&surface_props),
        ).expect("Unable to create surface");

        self.surface = Some(surface);
    }

    fn redraw(&mut self) {
        //println!("redraw");
        self.draw();

        //canvas.flush();
        //surface.flush();
        self.context.flush();
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
    let application = Application::new(|proc_name| platform.get_proc(proc_name));

    platform.run(application);
}
