use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, RequestAdapterError, RequestDeviceError, SurfaceCapabilities, SurfaceConfiguration, SurfaceError, TextureUsages};
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::rendering::instance::State;


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
               match state.render()
               {
                   Ok(r) => (),
                   Err(e) => eprintln!("Error: {}", e),
               }
                // Emits a new redraw requested event.
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) =>
            {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } =>
            {
                println!("MEME");
            }

            _ => (),
        }
    }
}
