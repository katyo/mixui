use core::marker::PhantomData;
use super::{HasContext};

#[cfg(feature = "glam")]
mod glam_impls;

#[cfg(feature = "stretch")]
mod stretch_impls;

#[cfg(feature = "colours")]
mod colours_impls;

/// Uniform location binding
pub struct Uniform<G: HasContext, T> {
    pub(super) location: Option<G::UniformLocation>,
    _type: PhantomData<T>,
}

impl<G: HasContext, T: AsUniform<G>> Uniform<G, T> {
    pub(super) fn new(location: Option<G::UniformLocation>) -> Self {
        Self { location: location, _type: PhantomData }
    }

    /// Load data to uniform location
    pub fn load(&self, gl: &G, data: T::Type) {
        if let Some(location) = &self.location {
            T::uniform_load(gl, location, data);
        }
    }
}

/// The trait for types which can be used as a uniform data
pub trait AsUniform<G: HasContext> {
    type Type;
    fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type);
}

macro_rules! as_uniform_impls {
    ($($type: ty, $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for $type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), data); }
                }
            }
        )*
    };
}

as_uniform_impls! {
    f32, uniform_1_f32;
    i32, uniform_1_i32;
}

macro_rules! as_uniform_impls_tuple {
    ($($type: ty, $func: ident, ($($arg: tt),+);)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for &$type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $(data.$arg),+); }
                }
            }
            impl<G: HasContext> AsUniform<G> for $type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $(data.$arg),+); }
                }
            }
        )*
    };
}

as_uniform_impls_tuple! {
    (f32, f32), uniform_2_f32, (0, 1);
    (f32, f32, f32), uniform_3_f32, (0, 1, 2);
    (f32, f32, f32, f32), uniform_4_f32, (0, 1, 2, 3);
    (i32, i32), uniform_2_i32, (0, 1);
    (i32, i32, i32), uniform_3_i32, (0, 1, 2);
    (i32, i32, i32, i32), uniform_4_i32, (0, 1, 2, 3);
}

macro_rules! as_uniform_impls_array_ref {
    ($($type: ty, $func: ident, $size: tt;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for &[$type; $size] {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), data); }
                }
            }
            impl<G: HasContext> AsUniform<G> for [$type; $size] {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), &data); }
                }
            }
        )*
    };
}

as_uniform_impls_array_ref! {
    f32, uniform_1_f32_slice, 1;
    f32, uniform_2_f32_slice, 2;
    f32, uniform_3_f32_slice, 3;
    f32, uniform_4_f32_slice, 4;
    i32, uniform_1_i32_slice, 1;
    i32, uniform_2_i32_slice, 2;
    i32, uniform_3_i32_slice, 3;
    i32, uniform_4_i32_slice, 4;
}

pub struct Direct<T>(T);
pub struct Transposed<T>(T);

macro_rules! as_uniform_impls_mat_array_ref {
    ($($conv:tt, $tran:tt, $type: ty, $func: ident, $size: tt;)*) => {
        $(
            impl<'a, G: HasContext> AsUniform<G> for $conv<&'a [$type; $size]> {
                type Type = &'a [$type; $size];
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $tran, data); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_array_ref! {
    Direct, false, f32, uniform_matrix_2_f32_slice, 4;
    Direct, false, f32, uniform_matrix_3_f32_slice, 9;
    Direct, false, f32, uniform_matrix_4_f32_slice, 16;
    Transposed, true, f32, uniform_matrix_2_f32_slice, 4;
    Transposed, true, f32, uniform_matrix_3_f32_slice, 9;
    Transposed, true, f32, uniform_matrix_4_f32_slice, 16;
}
