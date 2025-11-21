use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use wgpu::{Adapter, Device, Instance, Queue, RequestAdapterError, RequestDeviceError, SurfaceCapabilities, SurfaceConfiguration, SurfaceError, TextureUsages};
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};


use crate::rendering::instance::State;


//#[derive(Default)]
pub struct App
{
    state: Option<State>,
    shader_watcher: RecommendedWatcher
}
impl App
{
    pub fn new() -> Self
    {

        let shader_watcher = start_watching(PathBuf::from("assets/shaders/"));
        App 
        {
            state: None,
            shader_watcher
        }
    }
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
use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::{
    sync::{mpsc, Mutex},
    fs,
};
/// Creates a file system watcher that notifies when *any* WGSL in the directory changes.
fn start_watching(dir: PathBuf) -> RecommendedWatcher {
    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    }).expect("watcher");

    watcher
        .watch(&dir, RecursiveMode::Recursive)
        .expect("Failed to watch shader directory");

    // Spawn a thread to receive events:
    std::thread::spawn(move || {
        let mut last_event = Instant::now();
        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    // Only treat as "real" if enough time has passed
                    if last_event.elapsed() > Duration::from_millis(50) {
                        println!("{}",event.paths.first().unwrap().to_str().unwrap());
                        // Call your shader reload code here
                    }

                    last_event = Instant::now();
                }
                _ => {}
            }
        }
    });

    watcher
}
fn handle_reload()
{
    println!("Test");
}
