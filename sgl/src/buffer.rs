use core::{
    marker::PhantomData,
    mem::size_of,
    slice::from_raw_parts,
    ops::{RangeBounds, Bound},
};
use super::{Result, GL, HasContext, Attribs};

pub trait BufferTarget {
    /// OpenGL target
    const TARGET: u32;

    /// Data type
    type Type;
}

pub struct Array<T>(PhantomData<T>);

impl<T> BufferTarget for Array<T> {
    const TARGET: u32 = GL::ARRAY_BUFFER;
    type Type = T;
}

pub struct ElementArray<T: AsElement>(PhantomData<T>);

impl<T: AsElement> BufferTarget for ElementArray<T> {
    const TARGET: u32 = GL::ELEMENT_ARRAY_BUFFER;
    type Type = T;
}

pub trait AsElement {
    /// OpenGL data type of element
    const TYPE: u32 = 0;
}

macro_rules! as_element_impl {
    ($($type: ident, $gl_type: ident;)*) => {
        $(
            impl AsElement for $type {
                const TYPE: u32 = GL::$gl_type;
            }
        )*
    };
}

as_element_impl! {
    u8, UNSIGNED_BYTE;
    u16, UNSIGNED_SHORT;
    u32, UNSIGNED_INT;
}

pub struct Buffer<G: HasContext, T: BufferTarget> {
    pub(super) buffer: G::Buffer,
    pub(super) length: i32,
    _target: PhantomData<T>,
}

impl<G: HasContext, T: BufferTarget> Buffer<G, T> {
    /// Create buffer
    pub fn new(gl: &G) -> Result<Self> {
        unsafe {
            let buffer = gl.create_buffer()?;
            Ok(Self {
                buffer,
                length: 0,
                _target: PhantomData
            })
        }
    }

    /// Remove buffer
    pub fn del(self, gl: &G) {
        unsafe { gl.delete_buffer(self.buffer); }
    }

    /// Set buffer size
    pub fn resize(&mut self, gl: &G, size: usize) {
        self.bind_buffer(gl);
        self.length = size as i32;
        unsafe { gl.buffer_data_size(T::TARGET, self.length, GL::STATIC_DRAW); }
        self.unbind_buffer(gl);
    }

    /// Load data
    pub fn load(&mut self, gl: &G, data: &[T::Type]) {
        self.bind_buffer(gl);
        let size = data.len();
        self.length = size as i32;
        let raw: &[u8] = unsafe { from_raw_parts(
            data as *const _ as *const u8,
            size_of::<T::Type>() * size,
        ) };
        unsafe { gl.buffer_data_u8_slice(T::TARGET, raw, GL::STATIC_DRAW); }
        self.unbind_buffer(gl);
    }

    fn bind_buffer(&self, gl: &G) {
        unsafe { gl.bind_buffer(T::TARGET, Some(self.buffer)); }
    }

    fn unbind_buffer(&self, gl: &G) {
        unsafe { gl.bind_buffer(T::TARGET, None); }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawMode {
    Points = GL::POINTS,
    LineStrip = GL::LINE_STRIP,
    LineLoop = GL::LINE_LOOP,
    Lines = GL::LINES,
    TriangleStrip = GL::TRIANGLE_STRIP,
    TriangleFan = GL::TRIANGLE_FAN,
    Triangles = GL::TRIANGLES,
}

pub trait Draw<G: HasContext> {
    /// Draw part of attribs array of elements
    fn partial_draw(&self, gl: &G, mode: DrawMode, offset: i32, count: i32);

    /// The full data length in elements
    fn data_length(&self) -> i32;

    /// Draw full
    fn draw(&self, gl: &G, mode: DrawMode) {
        self.partial_draw(gl, mode, 0, self.data_length());
    }

    /// Draw range
    fn draw_range<R: RangeBounds<usize>>(&self, gl: &G, mode: DrawMode, range: R) {
        use self::Bound::*;

        let length = self.data_length();

        let start = match range.start_bound() {
            Included(start) => *start,
            Excluded(start) => *start + 1,
            Unbounded => 0,
        } as i32;

        if start > length {
            panic!("Attempt to exceed the length {} by start bound {}", length, start);
        }

        let end = match range.end_bound() {
            Included(end) => *end as i32 + 1,
            Excluded(end) => *end as i32,
            Unbounded => start + length,
        };

        if end > length {
            panic!("Attempt to exceed the length {} by end bound {}", length, start);
        }

        self.partial_draw(gl, mode, start, end - start);
    }
}

impl<G: HasContext, A: Attribs<G>, T> Draw<G> for (Buffer<G, Array<T>>, A) {
    fn partial_draw(&self, gl: &G, mode: DrawMode, offset: i32, count: i32) {
        self.0.bind_buffer(gl);
        self.1.enable_attribs(gl);
        unsafe { gl.draw_arrays(mode as u32, offset, count); }
        self.1.disable_attribs(gl);
        self.0.unbind_buffer(gl);
    }
    fn data_length(&self) -> i32 {
        self.0.length
    }
}

impl<G: HasContext, A: Attribs<G>, T, I: AsElement> Draw<G> for (Buffer<G, Array<T>>, A, Buffer<G, ElementArray<I>>) {
    fn partial_draw(&self, gl: &G, mode: DrawMode, offset: i32, count: i32) {
        self.0.bind_buffer(gl);
        self.2.bind_buffer(gl);
        self.1.enable_attribs(gl);
        unsafe { gl.draw_elements(mode as u32, count, I::TYPE, offset); }
        self.1.disable_attribs(gl);
        self.2.bind_buffer(gl);
        self.0.bind_buffer(gl);
    }
    fn data_length(&self) -> i32 {
        self.2.length
    }
}
