use wgpu::{RenderPass};
use crate::rendering::instance::State;

pub fn draw_calls(render_pass: &mut RenderPass, state: &State)
{
    state.tilemap_rendering.draw(render_pass, &state.main_camera_data);
}