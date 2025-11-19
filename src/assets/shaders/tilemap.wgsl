struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(3) index: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct CameraUniform
{
    pos: vec2<f32>,
    window_size: vec2<f32>
};
fn range_map(input: f32, inputStart: f32, inputEnd: f32, outputStart: f32, outputEnd: f32) -> f32
{
    var output = input - inputStart;
    output /= (inputEnd - inputStart);
    output *= (outputEnd - outputStart);
    return output + outputStart;
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput
{
    var tile_width = camera.window_size.x / 16.0;
    var tile_height = camera.window_size.y / 16.0;
    let index = in.index / 16.0;

    tile_width = range_map(tile_width, 0, camera.window_size.x, -1, 1);
    tile_height = range_map(tile_height, 0, camera.window_size.y, -1, 1);

    var out: VertexOutput;
    out.clip_position = vec4(in.position, 1.0, 1.0);


    //out.clip_position = vec4(in.position + in.index * 0.1, 1.0, 1.0);
    out.color = vec3(in.position, 0.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.1);
}