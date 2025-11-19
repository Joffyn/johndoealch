use bytemuck::cast_slice;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform
{
    pos: [f32; 2],
    //Probably shouldn't be here
    window_size: [f32; 2],
}
pub struct CameraData
{
    pub camera_bind_group: BindGroup,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: Buffer,
    pub camera_bind_group_layout: BindGroupLayout
}

impl CameraData
{
    pub fn new(device: &Device, window_size: &PhysicalSize<u32>) -> Self
    {
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let mut camera_uniform = CameraUniform
        {
            pos: [0.0, 0.0],
            window_size: [window_size.width as f32, window_size.height as f32]
        };
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        CameraData
        {
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            camera_bind_group_layout
        }
    }
    pub fn update_pos(&mut self, pos: [f32; 2], queue: &Queue)
    {
        self.camera_uniform.pos = pos;
        queue.write_buffer(&self.camera_buffer, 0, cast_slice(&[self.camera_uniform]));
    }
    pub fn resize(&mut self, window_size: PhysicalSize<u32>, queue: &Queue)
    {
        self.camera_uniform.window_size = [window_size.width as f32, window_size.height as f32];
        queue.write_buffer(&self.camera_buffer, 0, cast_slice(&[self.camera_uniform]));
    }
}