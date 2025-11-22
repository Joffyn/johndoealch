use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use notify::event::CreateKind;
use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use crate::app::{App, UserEvent};
use crate::tilemap::{TileMap, TileMapLoadConfig};

mod tilemap;
mod rendering;
mod app;



fn main()
{
    let tilemap = Arc::new(TileMap::new(TileMapLoadConfig::Box).unwrap());

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let _event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || shader_reloading(_event_loop_proxy));

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

fn shader_reloading(_event_loop_proxy: EventLoopProxy<UserEvent>)
{
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(50), None,tx).unwrap();
    debouncer.watch(Path::new("assets/shaders"), RecursiveMode::Recursive).unwrap();


    for result in rx 
    {
        match result 
        {
            Ok(events) => 
            {
                for ev in events
                {
                    if !matches!(ev.kind, EventKind::Create(_))
                    {
                        //println!("Not write, skipping over event");
                        continue;
                    }
                    for path in &ev.paths
                    {
                        let extension = match path.extension()
                        {
                            Some(ext) => ext.to_str().unwrap(), 
                            _ => "_",
                        };

                        if !extension.eq("wgsl")
                        {
                            //println!("Not wgsl shader, skipping");
                            continue;
                        }
                        let msg = path.file_name().unwrap().to_str().unwrap().to_owned();
                        _event_loop_proxy.send_event(UserEvent::ShaderReloaded(msg)).unwrap();
                    }

                }
            },
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }
}
