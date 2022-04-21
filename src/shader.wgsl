struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main() -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    return out;
}

// Fragment shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
