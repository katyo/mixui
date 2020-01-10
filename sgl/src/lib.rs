mod attrib;
mod uniform;
mod buffer;
mod texture;
mod program;
pub mod demo;

pub use glow::{self as GL, HasContext, Context};

pub use self::attrib::*;
pub use self::uniform::*;
pub use self::buffer::*;
pub use self::texture::*;
pub use self::program::*;

pub type Result<T> = std::result::Result<T, String>;

#[cfg(feature = "glam")]
pub use glam::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4};

#[cfg(feature = "stretch")]
pub use stretch::geometry::{Point, Size, Rect};

#[cfg(feature = "colours")]
pub use colours::{Rgb, Rgba, Hsv, Hsva, Hsl, Hsla};
