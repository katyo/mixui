use stretch::geometry::{Point, Size, Rect};
use super::{HasContext, AsUniform};

macro_rules! as_uniform_impls {
    ($($type: ty, ($($field:ident),+), $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for $type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $(data.$field),+); }
                }
            }
            impl<G: HasContext> AsUniform<G> for &$type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $(data.$field),+); }
                }
            }
        )*
    };
}

as_uniform_impls! {
    Point<f32>, (x, y), uniform_2_f32;
    Size<f32>, (width, height), uniform_2_f32;
    Rect<f32>, (start, end, top, bottom), uniform_4_f32;
}
