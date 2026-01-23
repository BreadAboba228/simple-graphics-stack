struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vertex_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.color = model.color;
    out.clip_pos = vec4<f32>(model.pos, 0.0, 1.0);

    return out;
}

@fragment
fn fragment_main(model: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(model.color, 1.0);
}
