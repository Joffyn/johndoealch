use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::rendering::instance::State;
use crate::rendering::material::Material;
use crate::rendering::shaderloading::{ShaderName, load_shader};

#[allow(dead_code)]
#[derive(Debug)]
pub enum UserEvent
{
    ShaderReloaded(String),
}


#[derive(Default)]
pub struct App
{
    state: Option<State>,
}
impl App
{
    fn test(&'static self)
    {
        self.state.as_ref().unwrap().load_all_shaders();
    }
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

                        //println!("Reload shader!");
                        //
                        //let mut_state: &mut State = self.state.as_mut().unwrap();
                        ////let state = self.state.as_ref().unwrap();
                        //let res = mut_state.tilemap_rendering.update_shader(&self.state.unwrap());
                        //match res
                        //{
                        //    Ok(s) => println!("{}", s),
                        //    Err(e) => eprintln!("{}", e),
                        //}
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
