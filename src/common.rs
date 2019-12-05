use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    // reference vertex positions with the co variable in vertex shaders
    #[sem(name = "vec", repr = "[f32; 2]", wrapper = "GLVertex")]
    Vertex,
    // reference vertex colors with the color variable in vertex shaders
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "GLVertexColor")]
    Color,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pub pos: GLVertex,
    pub rgb: GLVertexColor,
}
