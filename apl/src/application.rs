use std::path::PathBuf;

pub use winit::{
    event::{
        VirtualKeyCode as Key,
        MouseButton as Button,
        TouchPhase as Phase,
    },
    window::Icon,
};

pub use sgl::{HasContext};

/// Keyboard key name
/*
pub enum KeyCode {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
}
 */

#[derive(Debug, Clone, Copy)]
pub struct ViewConfig {
    pub color_bits: u8,
    pub alpha_bits: u8,
    pub stencil_bits: u8,
    pub depth_bits: u8,
    pub srgb_mode: bool,
    pub num_samples: u8,
    pub view_width: f32,
    pub view_height: f32,
    pub dot_scale: f32,
}

impl ViewConfig {
    pub fn new(
        color_bits: u8,
        alpha_bits: u8,
        stencil_bits: u8,
        depth_bits: u8,
        srgb_mode: bool,
        num_samples: u8,
        view_width: f32,
        view_height: f32,
        dot_scale: f32,
    ) -> Self {
        Self {
            color_bits,
            alpha_bits,
            stencil_bits,
            depth_bits,
            srgb_mode,
            num_samples,
            view_width,
            view_height,
            dot_scale,
        }
    }
}

/// Application callbacks
pub trait EventHandler {
    type Context: HasContext;

    /// Handle text input
    fn input(&mut self, _uchar: char) {}

    /// Handle key state chane
    fn key(&mut self, _key: Key, _state: bool) {}

    /// Handle pointer move
    fn pointer(&mut self, _x: f32, _y: f32) {}

    /// Handle button state change
    fn button(&mut self, _id: Button, _state: bool) {}

    /// Handle scroll
    fn scroll(&mut self, _dx: f32, _dy: f32, _pixelwize: bool) {}

    /// Handle touch
    fn touch(&mut self, _x: f32, _y: f32, _phase: Phase) {}

    /// Handle hover
    fn hover(&mut self, _state: bool) {}

    /// Handle focus
    fn focus(&mut self, _state: bool) {}

    /// Handle file over
    fn file_over(&mut self, _path: PathBuf) {}

    /// Handle file out
    fn file_out(&mut self) {}

    /// Handle file drop
    fn file_drop(&mut self, _path: PathBuf) {}

    /// Handle sizing
    fn reconf(&mut self, _conf: ViewConfig, _gl: &Self::Context) {}

    /// Handle drawing
    fn redraw(&mut self, _gl: &Self::Context) {}

    /// Destroy application
    fn destroy(&mut self) {}

    /// Suspend application
    fn suspend(&mut self) {}

    /// Resume application
    fn resume(&mut self) {}
}

/// Application config
pub struct AppConfig {
    /// Window title
    pub title: String,

    /// Window icon
    pub icon: Option<Icon>,
}
