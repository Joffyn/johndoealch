use std::collections::HashSet;
use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, RequestAdapterError, RequestDeviceError, SurfaceCapabilities, SurfaceConfiguration, SurfaceError, TextureUsages};
use winit::window::{Window};
use crate::rendering::game::camera::CameraData;
use crate::rendering::draw_calls::draw_calls;
use crate::rendering::game::tilemap_rendering::{TileInstance, TileMapRendering, TileMapVertex};
use crate::rendering::material::{Material, MaterialName, load_material};
use crate::rendering::shaderloading::ShaderName;

pub struct State
{
    pub window: Arc<Window>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub config: Arc<SurfaceConfiguration>,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub surface: Arc<Surface<'static>>,
    pub surface_format: wgpu::TextureFormat,
    pub swapchain_caps: SurfaceCapabilities,
    //Game specific data
    pub tilemap_rendering: TileMapRendering,
    pub main_camera_data: CameraData,

}
impl State
{
    pub fn load_all_shaders(&'static self)
    {
        let _ = load_material::<TileMapVertex, TileInstance>(
            &ShaderName::TileMap, 
            &MaterialName::TileMap, 
            &self.device, 
            &self.main_camera_data, 
            &self.swapchain_caps);
    }

    pub async fn new(window: Arc<Window>) -> Result<State, Box<dyn std::error::Error>>
    {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = get_default_adapter(&instance).await?;
        let (device, queue) = get_device_queue(&adapter).await?;

        let window_size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let config = SurfaceConfiguration
        {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: cap.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: cap.alpha_modes[0],
            view_formats: vec![],
        };
        let swapchain_caps = surface.get_capabilities(&adapter);

        let main_camera_data = CameraData::new(&device, &window_size);

        
        let tilemap_rendering = TileMapRendering::new(&device, &swapchain_caps, &main_camera_data);

        let state = State
        {
            window,
            device,
            queue,
            config,
            window_size,
            surface,
            surface_format,
            swapchain_caps,
            tilemap_rendering,
            main_camera_data,
        };

        // Configure surface for the first time
        state.configure_surface();

        Ok(state)
    }

    pub fn get_window(&self) -> &Window
    {
        &self.window
    }

    pub fn configure_surface(&self)
    {
        let surface_config = wgpu::SurfaceConfiguration
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.window_size.width,
            height: self.window_size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    {
        self.window_size = new_size;

        // reconfigure the surface
        self.configure_surface();
        self.main_camera_data.resize(new_size, &self.queue);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError>
    {
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()?;
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });


        // If you wanted to call any drawing commands, they would go here.
        draw_calls(&mut render_pass, &self);


        // End the renderpass.
        drop(render_pass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
        Ok(())
    }
}
async fn get_default_adapter(instance: &Instance)
    -> Result<Adapter, RequestAdapterError>
{
    instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
}
async fn get_device_queue(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError>
{
    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
}
