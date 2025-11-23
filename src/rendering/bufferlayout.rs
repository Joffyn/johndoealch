use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait BufferLayout
{
    fn desc<'a>() -> VertexBufferLayout<'static>;
}

