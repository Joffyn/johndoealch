use std::borrow::Cow;
use std::fs;
use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{vertex_attr_array,  Buffer, BufferAddress, BufferUsages, Device, MultisampleState, PipelineLayoutDescriptor,
           PrimitiveState,  RenderPass, RenderPipeline,  SurfaceCapabilities, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::rendering::game::camera::CameraData;
use crate::rendering::instance::State;
use crate::rendering::material::{Material, MaterialName};
use crate::rendering::bufferlayout::{BufferLayout};

const TILES_WIDE: u32 = 16;
const TILES_HIGH: u32 = 16;
const INSTANCES_LEN: u32 = TILES_WIDE * TILES_HIGH;



#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct TileMapVertex {
    pub pos: [f32; 2],
}

impl TileMapVertex
{
    const ATTRIBS: [VertexAttribute; 1] = vertex_attr_array![0 => Float32x2];
}
impl BufferLayout for TileMapVertex
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
const TILE : &[TileMapVertex] = &[
    TileMapVertex { pos: [0.0, 0.0]},
    TileMapVertex { pos: [0.0, 1.0]},
    TileMapVertex { pos: [1.0, 1.0]},
    TileMapVertex { pos: [1.0, 0.0]}
];
const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileInstance {
    index: [f32; 2],
}
impl BufferLayout for TileInstance
{
    fn desc() -> VertexBufferLayout<'static>
    {
        VertexBufferLayout
        {
            array_stride: size_of::<TileInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[VertexAttribute {offset: 0, shader_location: 3, format: VertexFormat::Float32x2}]
        }
    }
}

pub struct TileMapRendering
{
    //render_pipeline: RenderPipeline,
    material_name: MaterialName,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    instance_buffer: Buffer,
}

//impl Material for TileMapRendering
//{
//
//    fn update_shader(&mut self, state: &State)
//    -> Result<String, String>
//    {
//        let source = fs::read_to_string("assets/shaders/tilemap.wgsl").unwrap();
//    
//        validate_wgsl(source.as_str())?;
//
//
//        println!("Starting shader reloading");
//        let shader = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
//            label: None,
//            source: wgpu::ShaderSource::Wgsl(Cow::Owned(source)),
//        });
//        self.render_pipeline = state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
//            {
//            label: Some("Tilemap Pipeline"),
//            layout:
//            Some(&state.device.create_pipeline_layout(&PipelineLayoutDescriptor
//                {
//                label: Some("Layout"),
//                bind_group_layouts: &[&state.main_camera_data.camera_bind_group_layout],
//                push_constant_ranges: &[],
//            })),
//            vertex: wgpu::VertexState
//                {
//                module: &shader,
//                entry_point: Some("vs_main"), // 1.
//                buffers: &[TileMapVertex::desc(), TileInstance::desc()],// 2.
//                compilation_options: wgpu::PipelineCompilationOptions::default(),
//            },
//            fragment: Some(wgpu::FragmentState
//                { // 3.
//                module: &shader,
//                entry_point: Some("fs_main"),
//                targets: &[Some(state.swapchain_caps.formats[0].into())],
//                compilation_options: wgpu::PipelineCompilationOptions::default(),
//            }),
//            primitive: PrimitiveState::default(),
//            depth_stencil: None,
//            multisample: MultisampleState::default(),
//            multiview: None,
//            cache: None,
//        });
//        Ok(String::from("Succesfully loaded shader"))
//    }
//}

impl TileMapRendering
{
    pub fn new(device: &Device, swapchain_caps : &SurfaceCapabilities, camera_data: &CameraData) -> Self
    {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../assets/shaders/tilemap.wgsl"))),
        });
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(TILE),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let mut instances: Vec<TileInstance> = Vec::new();
        for y in 0..TILES_HIGH
        {
            for x in 0..TILES_WIDE
            {
                instances.push(TileInstance { index: [x as f32, y as f32]});
            }
        }
        let instance_buffer = device.create_buffer_init(
            &BufferInitDescriptor
            {
                label: Some("Instance buffer"),
                contents: cast_slice(instances.as_slice()),
                usage: BufferUsages::VERTEX

            }
        );
        TileMapRendering
        {
            //render_pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            //{
            //    label: Some("Tilemap Pipeline"),
            //    layout:
            //    Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
            //    {
            //        label: Some("Layout"),
            //        bind_group_layouts: &[&camera_data.camera_bind_group_layout],
            //        push_constant_ranges: &[],
            //    })),
            //    vertex: wgpu::VertexState
            //    {
            //        module: &shader,
            //        entry_point: Some("vs_main"), // 1.
            //        buffers: &[TileMapVertex::desc(), TileInstance::desc()],// 2.
            //        compilation_options: wgpu::PipelineCompilationOptions::default(),
            //    },
            //    fragment: Some(wgpu::FragmentState
            //    { // 3.
            //        module: &shader,
            //        entry_point: Some("fs_main"),
            //        targets: &[Some(swapchain_caps.formats[0].into())],
            //        compilation_options: wgpu::PipelineCompilationOptions::default(),
            //    }),
            //    primitive: PrimitiveState::default(),
            //    depth_stencil: None,
            //    multisample: MultisampleState::default(),
            //    multiview: None,
            //    cache: None,
            //}),
            material_name: MaterialName::TileMap,
            vertex_buffer,
            index_buffer,
            instance_buffer,
        }

    }
    //pub fn update_shader(&mut self, device: &Device, swapchain_caps : &SurfaceCapabilities, camera_data: &CameraData) 
    //-> Result<String, String>
    //{
    //    let source = fs::read_to_string("assets/shaders/tilemap.wgsl").unwrap();
    //
    //    validate_wgsl(source.as_str())?;


    //    println!("Starting shader reloading");
    //    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    //        label: None,
    //        source: wgpu::ShaderSource::Wgsl(Cow::Owned(source)),
    //    });
    //    self.render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
    //        {
    //        label: Some("Tilemap Pipeline"),
    //        layout:
    //        Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
    //            {
    //            label: Some("Layout"),
    //            bind_group_layouts: &[&camera_data.camera_bind_group_layout],
    //            push_constant_ranges: &[],
    //        })),
    //        vertex: wgpu::VertexState
    //            {
    //            module: &shader,
    //            entry_point: Some("vs_main"), // 1.
    //            buffers: &[TileMapVertex::desc(), TileInstance::desc()],// 2.
    //            compilation_options: wgpu::PipelineCompilationOptions::default(),
    //        },
    //        fragment: Some(wgpu::FragmentState
    //            { // 3.
    //            module: &shader,
    //            entry_point: Some("fs_main"),
    //            targets: &[Some(swapchain_caps.formats[0].into())],
    //            compilation_options: wgpu::PipelineCompilationOptions::default(),
    //        }),
    //        primitive: PrimitiveState::default(),
    //        depth_stencil: None,
    //        multisample: MultisampleState::default(),
    //        multiview: None,
    //        cache: None,
    //    });
    //    Ok(String::from("Succesfully loaded shader"))

    //}
    pub fn draw(&self, render_pass: &mut RenderPass, camera_data: &CameraData)
    {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &camera_data.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..INSTANCES_LEN as _);
    }
}
