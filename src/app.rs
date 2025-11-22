use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[allow(dead_code)]
#[derive(Debug)]
pub enum UserEvent
{
    ShaderReloaded(String),
}
use crate::rendering::instance::State;


#[derive(Default)]
pub struct App
{
    state: Option<State>,
}
impl App
{
    pub fn new() -> Self
    {

        App 
        {
            state: None,
        }
    }
}

impl ApplicationHandler<UserEvent> for App
{
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) 
    {
        match event 
        {
            UserEvent::ShaderReloaded(msg) => 
            {
                match msg.as_str()
                {
                    "tilemap.wgsl" => 
                    {
                        println!("Reload shader!");
                        let state: &mut State = self.state.as_mut().unwrap();
                        let res = state.tilemap_rendering.update_shader(
                            &state.device,
                            &state.swapchain_caps,
                            &state.main_camera_data);
                        match res
                        {
                            Ok(s) => println!("{}", s),
                            Err(e) => eprintln!("{}", e),
                        }
                    },
                    _ => println!("meme")
                }
            }
        }
    }
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
                    Err(e) => eprintln!("Error: {}", e),
                    _ => (),
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
