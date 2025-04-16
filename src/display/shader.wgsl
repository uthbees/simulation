struct GlobalUniform {
    window_size_px: vec2<f32>,
    camera_pos: vec2<f32>,
    camera_zoom: f32,
    // Padding for 16-bit alignment.
    padding: vec3<f32>
};

@group(0) @binding(0)
var<uniform> global_uniform: GlobalUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct TileInstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vert_main(
    model: VertexInput,
    instance: TileInstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color;

    var camera_coords: vec2<f32> = instance.position.xy + model.position.xy - global_uniform.camera_pos.xy;
    camera_coords = camera_coords.xy * global_uniform.camera_zoom;

    // Convert from pixel coordinates to normalized coordinates (-1 to 1).
    camera_coords.x = camera_coords.x / global_uniform.window_size_px.x * 2;
    camera_coords.y = camera_coords.y / global_uniform.window_size_px.y * 2;

    out.clip_position = vec4<f32>(camera_coords, 0.0, 1.0);

    return out;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
