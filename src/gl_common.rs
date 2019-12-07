use luminance_derive::{Semantics, Vertex};

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
