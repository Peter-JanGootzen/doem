use luminance_derive::{Semantics, Vertex, UniformInterface};
use luminance::shader::program::Uniform;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "pos", repr = "[f32; 3]", wrapper = "VertexPosition")]
    VertexPosition,
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "VertexColor")]
    Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub pos: VertexPosition,
    #[vertex(normalized = "true")]
    pub color: VertexColor,
}

// Create a uniform interface. This is a type that will be used to customize the shader. In our
// case, we just want to pass the time and the position of the triangle, for instance.
//
// This macro only supports structs for now; you cannot use enums as uniform interfaces.
#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    pub transform: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub view: Uniform<[[f32; 4]; 4]>,
}
