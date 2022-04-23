// Vertex shader

struct ModelMatInput {
    [[location(2)]] model_matrix_0: vec4<f32>;
    [[location(3)]] model_matrix_1: vec4<f32>;
    [[location(4)]] model_matrix_2: vec4<f32>;
    [[location(5)]] model_matrix_3: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] state: u32;
};

struct PVMat {
    m: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> pv_mat: PVMat;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>,
    [[location(1)]] state: u32,
    instance: ModelMatInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.position = pv_mat.m * model_matrix * vec4<f32>(position, 0.0, 1.0);
    out.state = state;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var mult: f32 = 1.0;
    if ((in.state & 1u) > 0u) {
        color = color + vec4<f32>(1.0, 1.0, 1.0, 1.0);
        mult = -1.0;
    }
    if ((in.state & 2u) > 0u) {
        color = color + vec4<f32>(0.1, 0.1, 0.1, 1.0) * mult;
    } 
    return color;
}