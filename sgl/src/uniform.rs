use core::marker::PhantomData;
use super::{HasContext};

/// Uniform location binding
pub struct Uniform<G: HasContext, T> {
    pub(super) location: G::UniformLocation,
    _type: PhantomData<T>,
}

impl<G: HasContext, T: AsUniform<G>> Uniform<G, T> {
    pub(super) fn new(location: G::UniformLocation) -> Self {
        Self { location: location, _type: PhantomData }
    }

    /// Load data to uniform location
    pub fn load(&self, gl: &G, data: T) {
        data.uniform_load(gl, &self.location);
    }
}

/// The trait for types which can be used as a uniform data
pub trait AsUniform<G: HasContext> {
    fn uniform_load(self, gl: &G, location: &G::UniformLocation);
}

macro_rules! as_uniform_impls {
    ($($type: ty, $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for $type {
                fn uniform_load(self, gl: &G, location: &G::UniformLocation) {
                    unsafe { gl.$func(Some(location.clone()), self); }
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
            impl<G: HasContext> AsUniform<G> for $type {
                fn uniform_load(self, gl: &G, location: &G::UniformLocation) {
                    unsafe { gl.$func(Some(location.clone()), $(self.$arg),+); }
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
                fn uniform_load(self, gl: &G, location: &G::UniformLocation) {
                    unsafe { gl.$func(Some(location.clone()), self); }
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

macro_rules! as_uniform_impls_mat_array_ref {
    ($($type: ty, $func: ident, $size: tt;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for (bool, &[$type; $size]) {
                fn uniform_load(self, gl: &G, location: &G::UniformLocation) {
                    unsafe { gl.$func(Some(location.clone()), self.0, self.1); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_array_ref! {
    f32, uniform_matrix_2_f32_slice, 4;
    f32, uniform_matrix_3_f32_slice, 9;
    f32, uniform_matrix_4_f32_slice, 16;
}
