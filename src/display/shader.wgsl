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
    out.clip_position = vec4<f32>(instance.position.xy + model.position.xy, 0.0, 1.0);
    return out;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
