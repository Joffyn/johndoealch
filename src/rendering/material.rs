use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;
use strum::{Display, EnumString};
use wgpu::{Device, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, SurfaceCapabilities, VertexBufferLayout, wgc::device};


use crate::rendering::{bufferlayout::BufferLayout, game::camera::CameraData, instance::State, shaderloading::{ShaderName, load_shader}};

#[derive(Eq, PartialEq, Hash, Debug, Display, EnumString, Copy, Clone)]
pub enum MaterialName
{
    TileMap,
}

static MATERIAL_MAP: Lazy<RwLock<HashMap<MaterialName, Material>>> = Lazy::new(|| 
{
    let map = HashMap::new();
    RwLock::new(map)
});
pub fn on_shader_reload(shader_name: &ShaderName)
{
    let mut mats = MATERIAL_MAP.write().unwrap();

    for (_, mat) in mats.iter_mut()
    {
        mat.update_shader(shader_name);
    }
    
}

pub fn load_material<T, I>(
    shader_name: &ShaderName,
    material_name: &MaterialName,
    device: &'static Device,
    camera_data: &'static CameraData,
    swapchain_caps: &'static SurfaceCapabilities) -> Result<MaterialName, Box<dyn std::error::Error>>
    where T: BufferLayout, I: BufferLayout
{
    let shader = load_shader(shader_name, device)?;
    let vertex_buffer_layout = T::desc();
    let instance_buffer_layout = I::desc();
    let mat = Material 
        {
        render_pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            {
            label: Some("Tilemap Pipeline"),
            layout:
            Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
                {
                label: Some("Layout"),
                bind_group_layouts: &[&camera_data.camera_bind_group_layout],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState
                {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[vertex_buffer_layout.clone(), instance_buffer_layout.clone()],// 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState
                { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(swapchain_caps.formats[0].into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        }),
        shader_name: shader_name.clone(),
        camera_data,
        swapchain_caps,
        device,
        vertex_buffer_layout,
        instance_buffer_layout
    };
    MATERIAL_MAP.write().unwrap().insert(material_name.clone(), mat);
    Ok(material_name.clone())
}
pub struct Material
{
    render_pipeline: RenderPipeline,
    shader_name: ShaderName,
    device: &'static Device,
    camera_data: &'static CameraData,
    swapchain_caps: &'static SurfaceCapabilities,
    vertex_buffer_layout: VertexBufferLayout<'static>,
    instance_buffer_layout: VertexBufferLayout<'static>
}

impl Material
{
    //pub fn new<T, I>(shader_name: &ShaderName, device: &'static Device, camera_data: &'static CameraData, swapchain_caps: &'static SurfaceCapabilities)
    //-> Result<Self, Box<dyn std::error::Error>>
    //where T: BufferLayout, I: BufferLayout
    //{
    //    let shader = load_shader(shader_name, device)?;
    //    let vertex_buffer_layout = T::desc();
    //    let instance_buffer_layout = I::desc();
    //    let mat = Material 
    //    {
    //        render_pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
    //        {
    //            label: Some("Tilemap Pipeline"),
    //            layout:
    //            Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor
    //            {
    //                label: Some("Layout"),
    //                bind_group_layouts: &[&camera_data.camera_bind_group_layout],
    //                push_constant_ranges: &[],
    //            })),
    //            vertex: wgpu::VertexState
    //            {
    //                module: &shader,
    //                entry_point: Some("vs_main"), // 1.
    //                buffers: &[vertex_buffer_layout.clone(), instance_buffer_layout.clone()],// 2.
    //                compilation_options: wgpu::PipelineCompilationOptions::default(),
    //            },
    //            fragment: Some(wgpu::FragmentState
    //            { // 3.
    //                module: &shader,
    //                entry_point: Some("fs_main"),
    //                targets: &[Some(swapchain_caps.formats[0].into())],
    //                compilation_options: wgpu::PipelineCompilationOptions::default(),
    //            }),
    //            primitive: PrimitiveState::default(),
    //            depth_stencil: None,
    //            multisample: MultisampleState::default(),
    //            multiview: None,
    //            cache: None,
    //        }),
    //        shader_name: shader_name.clone(),
    //        device,
    //        camera_data,
    //        swapchain_caps,
    //        vertex_buffer_layout,
    //        instance_buffer_layout
    //    };
    //    MATERIALS.write().unwrap().push(mat);
    //    Ok(MATERIALS.read().unwrap().get(0).unwrap())
    //}
    fn update_shader(&mut self, shader_name: &ShaderName)
    -> Result<String, String>
    {
        if !matches!(self.shader_name, shader_name) 
        {
            return Ok("Not the correct shader".to_string());
        }

        let shader = match load_shader(shader_name, self.device)
        {
            Ok(s) => s,
            Err(_) => return Err("Failed to update shader".to_string()),
        };


        self.render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
            {
            label: Some("Tilemap Pipeline"),
            layout:
            Some(&self.device.create_pipeline_layout(&PipelineLayoutDescriptor
                {
                label: Some("Layout"),
                bind_group_layouts: &[&self.camera_data.camera_bind_group_layout],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState
                {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[self.vertex_buffer_layout.clone(), self.instance_buffer_layout.clone()],// 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState
                { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(self.swapchain_caps.formats[0].into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        Ok(String::from("Succesfully loaded shader"))
    }
}

//pub trait Material
//{
//    //fn desc(&self) -> VertexBufferLayout<'static>;
//    //fn new(device: &Device, swapchain_caps : &SurfaceCapabilities, camera_data: &CameraData) -> Self;
//    //fn draw(&self, render_pass: &mut RenderPass, camera_data: &CameraData);
//    fn update_shader(&mut self, state: &State)
//    -> Result<String, String>;
//}

