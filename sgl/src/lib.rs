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
