use core::{
    marker::PhantomData,
    mem::size_of,
    slice::from_raw_parts,
};
use super::{Result, GL, HasContext, Uniform};

pub trait TextureTarget {
    /// OpenGL texture target
    const TARGET: u32;

    /// Texture image size type
    type Coords: Copy + IsPow2;

    /// Load image data to texture
    fn load_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, size: Self::Coords, data: Option<&[u8]>);

    /// Load image data to texture
    fn load_sub_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, off: Self::Coords, size: Self::Coords, data: Option<&[u8]>);
}

pub struct Texture2D;

impl TextureTarget for Texture2D {
    const TARGET: u32 = GL::TEXTURE_2D;
    type Coords = (usize, usize);

    fn load_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, size: Self::Coords, data: Option<&[u8]>) {
        if let Some(raw) = &data {
            if size.0 * size.1 * size_of::<F::Pixel>() != raw.len() {
                panic!("Texture image data size mismatch");
            }
        }
        unsafe {
            gl.tex_image_2d(
                Self::TARGET,
                level,
                F::FORMAT as i32,
                size.0 as i32,
                size.1 as i32,
                0,
                F::FORMAT,
                F::TYPE,
                data,
            );
        }
    }

    fn load_sub_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, off: Self::Coords, size: Self::Coords, data: Option<&[u8]>) {
        if let Some(raw) = &data {
            if size.0 * size.1 * size_of::<F::Pixel>() != raw.len() {
                panic!("Texture sub image data size mismatch");
            }
        }
        unsafe {
            gl.tex_sub_image_2d_u8_slice(
                Self::TARGET,
                level,
                off.0 as i32,
                off.1 as i32,
                size.0 as i32,
                size.1 as i32,
                F::FORMAT,
                F::TYPE,
                data,
            );
        }
    }
}

pub struct Texture3D;

impl TextureTarget for Texture3D {
    const TARGET: u32 = GL::TEXTURE_3D;
    type Coords = (usize, usize, usize);

    fn load_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, size: Self::Coords, data: Option<&[u8]>) {
        if let Some(raw) = &data {
            if size.0 * size.1 * size.2 * size_of::<F::Pixel>() != raw.len() {
                panic!("Texture image data size mismatch");
            }
        }
        unsafe {
            gl.tex_image_3d(
                Self::TARGET,
                level,
                F::FORMAT as i32,
                size.0 as i32,
                size.1 as i32,
                size.2 as i32,
                0,
                F::FORMAT,
                F::TYPE,
                data,
            );
        }
    }

    fn load_sub_image<F: TextureFormat, G: HasContext>(gl: &G, level: i32, off: Self::Coords, size: Self::Coords, data: Option<&[u8]>) {
        if let Some(raw) = &data {
            if size.0 * size.1 * size.2 * size_of::<F::Pixel>() != raw.len() {
                panic!("Texture sub image data size mismatch");
            }
        }
        unsafe {
            gl.tex_sub_image_3d_u8_slice(
                Self::TARGET,
                level,
                off.0 as i32,
                off.1 as i32,
                off.2 as i32,
                size.0 as i32,
                size.1 as i32,
                size.2 as i32,
                F::FORMAT,
                F::TYPE,
                data,
            );
        }
    }
}

/// Format of texture pixels
pub trait TextureFormat {
    type Pixel;
    const FORMAT: u32;
    const TYPE: u32;
}

macro_rules! texture_formats {
    ($($name: ident, $format: ident, $type: ident, $pixel: ty;)*) => {
        $(
            pub struct $name;

            impl TextureFormat for $name {
                type Pixel = $pixel;
                const FORMAT: u32 = GL::$format;
                const TYPE: u32 = GL::$type;
            }
        )+
    };
}

texture_formats! {
    RGB888, RGB, UNSIGNED_BYTE, (u8, u8, u8);
    RGBA8888, RGBA, UNSIGNED_BYTE, (u8, u8, u8, u8);
    L8, LUMINANCE, UNSIGNED_BYTE, u8;
    A8, ALPHA, UNSIGNED_BYTE, u8;
    LA88, LUMINANCE_ALPHA, UNSIGNED_BYTE, (u8, u8);

    RGB565, RGB, UNSIGNED_SHORT_5_6_5, u16;
    RGBA4444, RGBA, UNSIGNED_SHORT_4_4_4_4, u16;
    RGBA5551, RGBA, UNSIGNED_SHORT_5_5_5_1, u16;
}

pub struct Texture<G: HasContext, T: TextureTarget, F: TextureFormat> {
    pub(super) texture: G::Texture,
    _phantom: PhantomData<(T, F)>,
}

impl<G: HasContext, T: TextureTarget, F: TextureFormat> Texture<G, T, F> {
    /// Create texture
    pub fn new(gl: &G) -> Result<Self> {
        unsafe {
            let texture = gl.create_texture()?;
            Ok(Self { texture, _phantom: PhantomData })
        }
    }

    /// Delete texture
    pub fn del(self, gl: &G) {
        unsafe { gl.delete_texture(self.texture); }
    }

    pub fn init(&self, gl: &G, size: T::Coords) {
        self.bind_texture(gl);
        T::load_image::<F, G>(gl, 0, size, None);
        self.unbind_texture(gl);
    }

    pub fn load(&self, gl: &G, size: T::Coords, data: &[F::Pixel]) {
        self.bind_texture(gl);
        let raw = unsafe { from_raw_parts(
            data as *const _ as *const u8,
            size_of::<F::Pixel>() * data.len()
        ) };
        T::load_image::<F, G>(gl, 0, size, Some(raw));
        if size.is_pow2() {
            unsafe {
                gl.generate_mipmap(T::TARGET);
            }
        } else {
            unsafe {
                gl.tex_parameter_i32(T::TARGET, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
                gl.tex_parameter_i32(T::TARGET, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
                gl.tex_parameter_i32(T::TARGET, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
            }
        }
        self.unbind_texture(gl);
    }

    fn bind_texture(&self, gl: &G) {
        unsafe { gl.bind_texture(T::TARGET, Some(self.texture)); }
    }

    fn unbind_texture(&self, gl: &G) {
        unsafe { gl.bind_texture(T::TARGET, None); }
    }
}

pub trait Textures<G: HasContext> {
    fn enable(&self, gl: &G);
    fn disable(&self, gl: &G);
}

impl<G: HasContext, T: TextureTarget, F: TextureFormat> Textures<G> for (Texture<G, T, F>, Uniform<G, i32>) {
    fn enable(&self, gl: &G) {
        unsafe { gl.active_texture(GL::TEXTURE0); }
        self.0.bind_texture(gl);
        self.1.load(gl, GL::TEXTURE0 as i32);
    }

    fn disable(&self, gl: &G) {
        unsafe { gl.active_texture(GL::TEXTURE0); }
        self.0.unbind_texture(gl);
    }
}

macro_rules! textures_impls {
    ($($($Tx: ident, $Fx: ident, $x: tt, $TEXTUREx: ident),+;)*) => {
        $(
            impl<G: HasContext, $($Tx: TextureTarget, $Fx: TextureFormat),+> Textures<G> for ($((Texture<G, $Tx, $Fx>, Uniform<G, i32>)),+) {
                fn enable(&self, gl: &G) {
                    $(
                        unsafe { gl.active_texture(GL::$TEXTUREx); }
                        (self.$x).0.bind_texture(gl);
                        (self.$x).1.load(gl, GL::$TEXTUREx as i32);
                    )+
                }

                fn disable(&self, gl: &G) {
                    $(
                        unsafe { gl.active_texture(GL::$TEXTUREx); }
                        (self.$x).0.unbind_texture(gl);
                    )+
                }
            }
        )*
    };
}

textures_impls! {
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3, T4, F4, 4, TEXTURE4;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3, T4, F4, 4, TEXTURE4, T5, F5, 5, TEXTURE5;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3, T4, F4, 4, TEXTURE4, T5, F5, 5, TEXTURE5, T6, F6, 6, TEXTURE6;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3, T4, F4, 4, TEXTURE4, T5, F5, 5, TEXTURE5, T6, F6, 6, TEXTURE6, T7, F7, 7, TEXTURE7;
    T0, F0, 0, TEXTURE0, T1, F1, 1, TEXTURE1, T2, F2, 2, TEXTURE2, T3, F3, 3, TEXTURE3, T4, F4, 4, TEXTURE4, T5, F5, 5, TEXTURE5, T6, F6, 6, TEXTURE6, T7, F7, 7, TEXTURE7, T8, F8, 8, TEXTURE8;
}

pub trait IsPow2 {
    fn is_pow2(&self) -> bool;
}

impl IsPow2 for (usize, usize) {
    fn is_pow2(&self) -> bool {
        self.0.is_pow2() && self.1.is_pow2()
    }
}

impl IsPow2 for (usize, usize, usize) {
    fn is_pow2(&self) -> bool {
        self.0.is_pow2() && self.1.is_pow2() && self.2.is_pow2()
    }
}

impl IsPow2 for usize {
    fn is_pow2(&self) -> bool {
        (self & (self - 1)) == 0
    }
}
