// Vertex shader

struct ModelMatInput {
    [[location(2)]] model_matrix_0: vec4<f32>;
    [[location(3)]] model_matrix_1: vec4<f32>;
    [[location(4)]] model_matrix_2: vec4<f32>;
    [[location(5)]] model_matrix_3: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] state: f32;
};

struct PVMat {
    m: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> pv_mat: PVMat;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>,
    [[location(1)]] state: f32,
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
    if (in.state > 0.0) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}