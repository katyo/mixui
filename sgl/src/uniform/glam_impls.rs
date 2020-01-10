use glam::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4};
use super::{HasContext, AsUniform, Direct, Transposed};

macro_rules! as_uniform_impls_as_ref {
    ($($type: ty, $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for $type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), data.as_ref()); }
                }
            }
            impl<G: HasContext> AsUniform<G> for &$type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), data.as_ref()); }
                }
            }
        )*
    };
}

as_uniform_impls_as_ref! {
    Vec2, uniform_2_f32_slice;
    Vec3, uniform_3_f32_slice;
    Vec4, uniform_4_f32_slice;
}

macro_rules! as_uniform_impls_mat_as_ref {
    ($($conv:tt, $tran:tt, $type: ty, $func: ident;)*) => {
        $(
            impl<'a, G: HasContext> AsUniform<G> for $conv<&'a $type> {
                type Type = &'a $type;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $tran, data.as_ref()); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_as_ref! {
    Direct, false, Mat2, uniform_matrix_2_f32_slice;
    Direct, false, Mat4, uniform_matrix_4_f32_slice;
    Transposed, true, Mat2, uniform_matrix_2_f32_slice;
    Transposed, true, Mat4, uniform_matrix_4_f32_slice;
}

macro_rules! as_uniform_impls_mat_as_ref_pure {
    ($($type: ty, $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for &$type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), false, data.as_ref()); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_as_ref_pure! {
    Mat2, uniform_matrix_2_f32_slice;
    Mat4, uniform_matrix_4_f32_slice;
}

macro_rules! as_uniform_impls_mat_cols_array {
    ($($conv: tt, $tran: tt, $type: ty, $func: ident;)*) => {
        $(
            impl<'a, G: HasContext> AsUniform<G> for $conv<&'a $type> {
                type Type = &'a $type;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), $tran, &data.to_cols_array()); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_cols_array! {
    Direct, false, Mat3, uniform_matrix_3_f32_slice;
    Transposed, true, Mat3, uniform_matrix_3_f32_slice;
}

macro_rules! as_uniform_impls_mat_cols_array_pure {
    ($($type: ty, $func: ident;)*) => {
        $(
            impl<G: HasContext> AsUniform<G> for &$type {
                type Type = Self;
                fn uniform_load(gl: &G, location: &G::UniformLocation, data: Self::Type) {
                    unsafe { gl.$func(Some(location.clone()), false, &data.to_cols_array()); }
                }
            }
        )*
    };
}

as_uniform_impls_mat_cols_array_pure! {
    Mat3, uniform_matrix_3_f32_slice;
}
