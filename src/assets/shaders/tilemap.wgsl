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
    //Change hardcoded values here
    var tile_width = camera.window_size.x / 16.0;
    var tile_height = camera.window_size.y / 16.0;
    let index = in.index / 16.0;

    //Map to clipspace(-1 to 1) and add 1
    //Which is why it's 0, 2 here
    tile_width = range_map(tile_width, 0, camera.window_size.x, 0, 2);
    tile_height = range_map(tile_height, 0, camera.window_size.y, 0, 2);

    var out: VertexOutput;
    
    //Multiply by the size of the tile
    var quad = in.position;
    quad.x *= tile_width;
    quad.y *= tile_height;
    //Subtract 1 in order to get the tile into the bottomleft corner
    quad -= 1.0;
    //Multiply by 2 as size is scaled by 2
    quad += index * 2.0;


    out.clip_position = vec4(quad, 1.0, 1.0);

    out.color = vec3(in.position, 0.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
