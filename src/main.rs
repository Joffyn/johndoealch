use std::sync::Arc;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::rendering::instance::App;
use crate::tilemap::{TileMap, TileMapLoadConfig};

mod tilemap;
mod rendering;

fn main()
{
    let tilemap = Arc::new(TileMap::new(TileMapLoadConfig::Box).unwrap());

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();

}
