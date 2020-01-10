use colours::{Rgb, Rgba, Hsv, Hsva, Hsl, Hsla};
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
    Rgb<f32>, (red, green, blue), uniform_3_f32;
    Rgba<f32>, (red, green, blue, alpha), uniform_4_f32;
    Hsv<f32>, (hue, saturation, value), uniform_3_f32;
    Hsva<f32>, (hue, saturation, value, alpha), uniform_4_f32;
    Hsl<f32>, (hue, saturation, lightness), uniform_3_f32;
    Hsla<f32>, (hue, saturation, lightness, alpha), uniform_4_f32;
}
