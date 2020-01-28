use super::{Result, GL, HasContext, AsAttrib, Attrib, AsUniform, Uniform};

#[repr(u32)]
pub enum ShaderType {
    Vertex = GL::VERTEX_SHADER,
    Fragment = GL::FRAGMENT_SHADER,
}

/// GLSL Shader
pub struct Shader<G: HasContext> {
    pub(super) shader: G::Shader,
}

impl<G: HasContext> Shader<G> {
    /// Create shader of specified type using specified source
    pub fn new<S: AsRef<str>>(gl: &G, typ: ShaderType, src: S) -> Result<Self> {
        unsafe {
            let shader = gl.create_shader(typ as u32)?;
            gl.shader_source(shader, src.as_ref());
            gl.compile_shader(shader);
            if gl.get_shader_compile_status(shader) {
                Ok(Self { shader })
            } else {
                let error = gl.get_shader_info_log(shader);
                gl.delete_shader(shader);
                Err(error)
            }
        }
    }

    /// Delete shader
    ///
    /// NOTE: We cannot simply use `Drop` trait because we need context for deleting shader object
    /// NOTE2: Usually we dont need to do it manually
    pub fn del(self, gl: &G) {
        unsafe { gl.delete_shader(self.shader); }
    }
}

/// GLSL Program
pub struct Program<G: HasContext> {
    pub(super) program: G::Program,
}

impl<G: HasContext> Program<G> {
    /// Create program using specified shaders
    pub fn new(gl: &G, shaders: Vec<Shader<G>>) -> Result<Self> {
        unsafe {
            let program = gl.create_program()?;
            for shader in &shaders {
                gl.attach_shader(program, shader.shader);
            }
            gl.link_program(program);
            let res = if gl.get_program_link_status(program) {
                Ok(Self { program })
            } else {
                Err(gl.get_program_info_log(program))
            };
            for shader in shaders.into_iter() {
                gl.detach_shader(program, shader.shader);
                shader.del(gl);
            }
            if res.is_err() {
                gl.delete_program(program);
            }
            res
        }
    }

    /// Delete program
    ///
    /// NOTE: we cannot simply use `Drop` trait because we need context for deleting program object
    pub fn del(self, gl: &G) {
        unsafe { gl.delete_program(self.program); }
    }

    /// Use program for rendering
    pub fn enable(&self, gl: &G) {
        unsafe { gl.use_program(Some(self.program)); }
    }

    /// Unuse program
    pub fn disable(&self, gl: &G) {
        unsafe { gl.use_program(None); }
    }

    pub fn uniform<T: AsUniform<G>, S: AsRef<str>>(&self, gl: &G, name: S) -> Uniform<G, T> {
        let name = name.as_ref();
        let location = unsafe { gl.get_uniform_location(self.program, name) };

        if location.is_none() {
            eprintln!("No uniform `{}` found", name);
        }

        Uniform::new(location)
    }

    pub fn attrib<T: AsAttrib<G>, S: AsRef<str>>(&self, gl: &G, name: S) -> Attrib<G, T> {
        let name = name.as_ref();
        let location = unsafe { gl.get_attrib_location(self.program, name) };

        if location.is_none() {
            eprintln!("No attribute `{}` found", name)
        }

        Attrib::new(location)
    }
}
