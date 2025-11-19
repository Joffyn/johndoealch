use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, IndexFormat, MultisampleState,
           PipelineLayoutDescriptor, PrimitiveState, RenderPass, RenderPipeline, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::DeviceExt;
use crate::rendering::instance::State;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
struct TestVertex {
    pub pos: [f32; 2],
}

impl TestVertex
{
    const ATTRIBS: [VertexAttribute; 1]
    = vertex_attr_array![0 => Float32x2];

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
const VERTICES: &[TestVertex] = &[
    TestVertex { pos: [0.0, 0.0]},
    TestVertex { pos: [0.0, 0.1]},
    TestVertex { pos: [0.1, 0.1]},
    TestVertex { pos: [0.1, 0.0]}
];
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];
fn test_render_pipeline(state: &State) -> RenderPipeline
{
    // Load the shaders from disk
    let shader = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../assets/shaders/test.wgsl"))),
    });
    state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
    {
        label: Some("Test Pipeline"),
        layout:
        Some(&state.device.create_pipeline_layout(&PipelineLayoutDescriptor
        {
            label: Some("Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })),
        vertex: wgpu::VertexState
        {
            module: &shader,
            entry_point: Some("vs_main"), // 1.
            buffers: &[TestVertex::desc()], // 2.
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState
        { // 3.
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(state.swapchain_caps.formats[0].into())],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
fn draw_triangle(render_pass: &mut RenderPass, state: &State)
{
    let pipeline = test_render_pipeline(state);
    let vertex_buffer = state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    let index_buffer = state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    let num_indices = INDICES.len() as u32;

    render_pass.set_pipeline(&pipeline);
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_pass.set_index_buffer(index_buffer.slice(..),IndexFormat::Uint16);
    render_pass.draw_indexed(0..num_indices, 0, 0..1);
}
