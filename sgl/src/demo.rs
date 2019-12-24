use super::{Result, HasContext, ShaderType, Shader, Program, Uniform, Attribs, Buffer, Array, Draw, DrawMode, Attrib};

pub struct Demo<G: HasContext> {
    program: Program<G>,
    uniform: Uniform<G, (f32, f32)>,
    geometry: (Buffer<G, Array<(f32, f32)>>, Attrib<G, (f32, f32)>),
    offset: (f32, f32),
}

impl<G: HasContext> Demo<G> {
    pub fn new(gl: &G) -> Result<Self> {
        let vertex_shader = Shader::new(gl, ShaderType::Vertex, include_str!("./demo.vert.glsl"))?;
        let fragment_shader = Shader::new(gl, ShaderType::Fragment, include_str!("./demo.frag.glsl"))?;
        let program = Program::new(gl, vec![vertex_shader, fragment_shader])?;

        let attrib = program.attrib(gl, "position")?;
        let uniform = program.uniform(gl, "offset")?;

        let mut vertex = attrib.buffer(gl)?;
        vertex.load(gl, &[
            (0.0, 0.5),
            (-0.5, -0.5),
            (0.5, -0.5),
        ]);
        let offset = (0.0, 0.0);

        Ok(Self { program, uniform, geometry: (vertex, attrib), offset })
    }

    pub fn del(self, gl: &G) {
        self.program.del(gl);
        self.geometry.0.del(gl);
    }

    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset = (x, y);
    }

    pub fn render(&self, gl: &G) {
        self.program.enable(gl);
        self.uniform.load(gl, self.offset);
        self.geometry.draw(gl, DrawMode::Triangles);
        self.program.disable(gl);
    }
}
