use std::fmt::Display;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::path::Path;
use std::process::Command;

pub enum ShaderKind
{
    Vertex,
    Pixel,
}

pub fn compile_shader(shaderkind: ShaderKind, path: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let output_path = Path::new(path).with_extension("spv");
    let shader = match shaderkind
    {
        ShaderKind::Vertex => "vs_6_0",
        ShaderKind::Pixel => "ps_6_0",
        _ => todo!(),
    };
    let status = Command::new("dxc")
        .args([
            "-T", shader,      // target: vertex shader (adjust per file)
            "-E", "main",        // entry point
            "-spirv",            // output SPIR-V
            "-Fo", output_path.to_str().unwrap(),
            path,
        ])
        .status()?;

    if status.success() {
        println!("✅ Compiled {path} -> {}", output_path.display());
    } else {
        eprintln!("❌ Failed to compile {path}");
    }
    Ok(())
}
//pub fn load_shader(spirv: Vec<u32>, device: Arc<Device>) -> Result<Arc<ShaderModule>, Validated<VulkanError>>
//{
//    let createinfo = ShaderModuleCreateInfo::new(spirv.as_slice());
//    unsafe { ShaderModule::new(device.clone(), createinfo) }
//}
