use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use strum::{Display, EnumString};

use naga::front::wgsl::Frontend as WgslParser;
use naga::valid::{Validator, ValidationFlags, Capabilities};
use once_cell::sync::Lazy;
use wgpu::{Device, ShaderModule};

#[derive(Eq, PartialEq, Hash, Debug, Display, EnumString, Copy, Clone)]
pub enum ShaderName
{
    TileMap,
    Test,
}

const SHADER_DIR: &str = "assets/shaders/";
static SHADER_MAP: Lazy<RwLock<HashMap<ShaderName, ShaderModule>>> = Lazy::new(|| 
{
    let map = HashMap::new();
    RwLock::new(map)
});
pub fn get_shader(shader_name: &ShaderName) -> Option<ShaderModule>
{
    SHADER_MAP.read().unwrap().get(shader_name).cloned()
}

pub fn reload_shader(shader_name: &ShaderName, device: &Device) 
-> Result<ShaderModule, Box<dyn std::error::Error>>
{
    let path = PathBuf::from(SHADER_DIR.to_string() + &shader_name.to_string().to_lowercase() + ".wgsl");
    let src = fs::read_to_string(path)?;
    validate_wgsl(src.as_str())?;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(src)),
    });
    SHADER_MAP.write().unwrap().insert(*shader_name, shader.clone());
    Ok(shader.clone())

}
pub fn load_shader(shader_name: &ShaderName, device: &Device) -> Result<ShaderModule, Box<dyn std::error::Error>>
{
    match get_shader(shader_name)
    {
        Some(s) => return Ok(s),
        _ => ()
    }

    let path = PathBuf::from(SHADER_DIR.to_string() + &shader_name.to_string().to_lowercase() + ".wgsl");
    let src = fs::read_to_string(path)?;
    validate_wgsl(src.as_str())?;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(src)),
    });
    SHADER_MAP.write().unwrap().insert(*shader_name, shader.clone());
    Ok(shader.clone())
}


fn validate_wgsl(source: &str) -> Result<(), String> 
{
    // Parse WGSL
    let module = WgslParser::new()
        .parse(source)
        .map_err(|e| format!("WGSL parse error: {:?}", e))?;

    // Validate semantic correctness
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());

    validator.validate(&module)
        .map_err(|e| format!("WGSL validation error: {:?}", e))?;

    Ok(())
}
