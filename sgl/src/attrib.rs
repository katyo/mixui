use core::{
    mem::size_of,
    marker::PhantomData
};
use super::{Result, GL, HasContext, Buffer, Array};

#[cfg(feature = "glam")]
mod glam_impls;

#[cfg(feature = "stretch")]
mod stretch_impls;

#[cfg(feature = "colours")]
mod colours_impls;

/// Attribute location binding
pub struct Attrib<G: HasContext, T: AsAttrib<G>> {
    pub(super) attrib: u32,
    _gl: PhantomData<(G, T)>,
}

impl<G: HasContext, T: AsAttrib<G>> Attrib<G, T> {
    pub(super) fn new(attrib: u32) -> Self {
        Self { attrib, _gl: PhantomData }
    }

    /// Enable attribute
    fn enable_attrib(&self, gl: &G) {
        unsafe { gl.enable_vertex_attrib_array(self.attrib); }
    }

    /// Disable attribute
    fn disable_attrib(&self, gl: &G) {
        unsafe { gl.disable_vertex_attrib_array(self.attrib); }
    }

    /// Setup attribute pointer
    pub fn pointer(&self, gl: &G, offset: i32, stride: i32) {
        T::attrib_pointer(gl, self.attrib, offset, stride);
    }
}

pub trait Attribs<G: HasContext> {
    /// Attributes type
    type Type;

    /// Enable attributes and setup attribute pointers
    fn enable_attribs(&self, gl: &G);

    /// Disable attributes
    fn disable_attribs(&self, gl: &G);

    /// Create suitable buffer
    fn buffer(&self, gl: &G) -> Result<Buffer<G, Array<Self::Type>>> {
        Buffer::new(gl)
    }
}

macro_rules! bytelen {
    ($($type:ty),*) => {
        ( $( size_of::<$type>() +)* 0) as i32
    };
}

impl<G: HasContext, T: AsAttrib<G>> Attribs<G> for Attrib<G, T> {
    type Type = T;

    fn enable_attribs(&self, gl: &G) {
        self.enable_attrib(gl);
        self.pointer(gl, 0, bytelen!(T));
    }

    fn disable_attribs(&self, gl: &G) {
        self.disable_attrib(gl);
    }
}

macro_rules! attribs_impls {
    ($($($Tx: ident, $x: tt),+;)*) => {
        $(
            impl<G: HasContext, $($Tx: AsAttrib<G>),+> Attribs<G> for ($(Attrib<G, $Tx>),+) {
                type Type = ($($Tx),+);

                fn enable_attribs(&self, gl: &G) {
                    $(self.$x.enable_attrib(gl);)+

                    let stride = bytelen!($($Tx),+);

                    let _offset = 0;
                    $(
                        self.$x.pointer(gl, _offset, stride);
                        let _offset = _offset + bytelen!($Tx);
                    )+
                }

                fn disable_attribs(&self, gl: &G) {
                    $(self.$x.disable_attrib(gl);)+
                }
            }
        )*
    };
}

attribs_impls! {
    T0, 0, T1, 1;
    T0, 0, T1, 1, T2, 2;
    T0, 0, T1, 1, T2, 2, T3, 3;
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4;
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5;
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6;
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7;
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8;
}

/// The trait for types which can be used as a vertex attribute
pub trait AsAttrib<G> {
    fn attrib_pointer(gl: &G, attrib: u32, offset: i32, stride: i32);
}

macro_rules! as_attrib_impls_f32 {
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

macro_rules! as_attrib_impls_i32 {
    ($($type: ty, $gl_type: ident, $size: tt;)*) => {
        $(
            impl<G: HasContext> AsAttrib<G> for $type {
                fn attrib_pointer(gl: &G, attrib: u32, offset: i32, stride: i32) {
                    unsafe { gl.vertex_attrib_pointer_i32(attrib, $size, GL::$gl_type, stride, offset); }
                }
            }
        )*
    };
}

as_attrib_impls_f32! {
    f32, FLOAT, 1, false;
    (f32, f32), FLOAT, 2, false;
    (f32, f32, f32), FLOAT, 3, false;
    (f32, f32, f32, f32), FLOAT, 4, false;
}

as_attrib_impls_i32! {
    i8, BYTE, 1;
    (i8, i8), BYTE, 2;
    (i8, i8, i8), BYTE, 3;
    (i8, i8, i8, i8), BYTE, 4;
    u8, UNSIGNED_BYTE, 1;
    (u8, u8), UNSIGNED_BYTE, 2;
    (u8, u8, u8), UNSIGNED_BYTE, 3;
    (u8, u8, u8, u8), UNSIGNED_BYTE, 4;
    i16, SHORT, 1;
    (i16, i16), SHORT, 2;
    (i16, i16, i16), SHORT, 3;
    (i16, i16, i16, i16), SHORT, 4;
    u16, UNSIGNED_SHORT, 1;
    (u16, u16), UNSIGNED_SHORT, 2;
    (u16, u16, u16), UNSIGNED_SHORT, 3;
    (u16, u16, u16, u16), UNSIGNED_SHORT, 4;
}
