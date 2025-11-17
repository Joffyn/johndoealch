use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait Vertex
{
    fn desc<'a>() -> VertexBufferLayout<'static>;
}


#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct TileMapVertex {
    pub pos: [f32; 2],
}

impl TileMapVertex
{
    const ATTRIBS: [VertexAttribute; 1]
    = vertex_attr_array![0 => Float32x2];
}
impl Vertex for TileMapVertex
{
    fn desc() -> VertexBufferLayout<'static>
    {
        VertexBufferLayout
        {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }

    }
}

