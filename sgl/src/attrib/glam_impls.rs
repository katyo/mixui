use glam::{Vec2, Vec3, Vec4};
use crate::{HasContext, GL, AsAttrib};

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
    Vec2, FLOAT, 2, false;
    Vec3, FLOAT, 3, false;
    Vec4, FLOAT, 4, false;
}
