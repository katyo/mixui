use colours::{Rgb, Rgba, Hsv, Hsva, Hsl, Hsla};
use super::{HasContext, GL, AsAttrib};

macro_rules! as_attrib_impls_vec {
    ($($type: ty, $gl_type: ident, $size: tt, $norm: ident;)*) => {
        $(
            impl<G: HasContext> AsAttrib<G> for $type {
                fn attrib_pointer(gl: &G, attrib: u32, offset: i32, stride: i32) {
                    unsafe { gl.vertex_attrib_pointer_f32(attrib, $size, GL::$gl_type, $norm, stride, offset); }
                }
            }
        )*
    };
}

as_attrib_impls_vec! {
    Rgb<f32>, FLOAT, 3, false;
    Rgba<f32>, FLOAT, 4, false;
    Hsv<f32>, FLOAT, 3, false;
    Hsva<f32>, FLOAT, 4, false;
    Hsl<f32>, FLOAT, 3, false;
    Hsla<f32>, FLOAT, 4, false;
}
