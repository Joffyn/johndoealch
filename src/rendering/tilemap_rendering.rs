use std::borrow::Cow;
use wgpu::{Device, Face, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, ShaderModule, SurfaceConfiguration};
use crate::rendering::vertex::{TileMapVertex, Vertex};
use crate::tilemap::Tile;

pub struct TileMapRendering
{
    render_pipeline: RenderPipeline,
    tiles_wide: u32,
    tiles_high: u32,
}
impl TileMapRendering
{
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        ) -> Self
    {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../assets/shaders/tilemap.wgsl"))),
        });
        TileMapRendering
        {
            render_pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            {
                label: Some("Tilemap Pipeline"),
                layout:
                Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
                {
                    label: Some("Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                })),
                vertex: wgpu::VertexState
                {
                    module: &shader,
                    entry_point: Some("vs_main"), // 1.
                    buffers: &[TileMapVertex::desc()], // 2.
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState
                { // 3.
                    module: &shader,
                    entry_point: Some("fs_main"),
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
            }),
            tiles_wide: 8,
            tiles_high: 8
        }
    }
    pub fn draw(visible_tiles: Vec<Tile>)
    {
        
    }
}
