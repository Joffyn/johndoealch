use std::sync::*;

use std::fmt::Display;
use wgpu::{Device, Face, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, ShaderModule, SurfaceConfiguration};
use crate::rendering::vertex::Vertex;

pub struct MaterialInstance
{
    pub base_material: Arc<Material>
}
pub struct Material
{
    pub render_pipeline: RenderPipeline
}
impl Material
{
    pub fn new<T>(
        vertex_shader: &ShaderModule,
        pixel_shader: &ShaderModule,
        device: &Device,
        config: &SurfaceConfiguration) -> Material
    where T : Vertex,
    {
        Material
        {
            render_pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            {
                label: Some("Render Pipeline"),
                layout:
                    Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
                    {
                        label: Some("Layout"),
                        bind_group_layouts: &[],
                        push_constant_ranges: &[],
                    })),
                vertex: wgpu::VertexState
                {
                    module: &vertex_shader,
                    entry_point: Some("main"), // 1.
                    buffers: &[T::desc()], // 2.
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState
                { // 3.
                    module: &pixel_shader,
                    entry_point: Some("main"),
                    targets: &[Some(wgpu::ColorTargetState
                    { // 4.
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: PrimitiveState
                {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState
                {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        }
    }
}
