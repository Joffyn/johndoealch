use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, RequestAdapterError, RequestDeviceError, SurfaceConfiguration, SurfaceError, TextureUsages};
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::rendering::tilemap_rendering::TileMapRendering;

struct State
{
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    tilemap_rendering: TileMapRendering
}
impl State
{
    async fn new(window: Arc<Window>) -> Result<State, Box<dyn std::error::Error>>
    {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = get_default_adapter(&instance).await?;
        let (device, queue) = get_device_queue(&adapter).await?;

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let config = SurfaceConfiguration
        {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: cap.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: cap.alpha_modes[0],
            view_formats: vec![],
        };

        let tilemap_rendering = TileMapRendering::new(&device, &config);

        let state = State
        {
            window,
            device,
            queue,
            config,
            size,
            surface,
            surface_format,
            tilemap_rendering
        };

        // Configure surface for the first time
        state.configure_surface();

        Ok(state)
    }

    fn get_window(&self) -> &Window
    {
        &self.window
    }

    fn configure_surface(&self)
    {
        let surface_config = wgpu::SurfaceConfiguration
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    fn render(&mut self) -> Result<(), SurfaceError>
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

        //basic_draw_call(&mut render_pass, &create_triangle(&self.device, &self.config));

        // If you wanted to call any drawing commands, they would go here.

        // End the renderpass.
        drop(render_pass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
        Ok(())
    }
}
#[derive(Default)]
pub struct App
{
    state: Option<State>,
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone())).expect("Could't create state");
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent)
    {
        let state = match &mut self.state
        {
            Some(canvas) => canvas,
            None => return,
        };
        match event
        {
            WindowEvent::CloseRequested =>
            {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested =>
            {
                state.render();
                // Emits a new redraw requested event.
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) =>
            {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            _ => (),
        }
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
