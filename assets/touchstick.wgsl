#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CircleMaterial {
    @location(0) color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> input: CircleMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 2.0 - 1.0;
    let alpha = 1.0 - pow(sqrt(dot(uv, uv)), 100.0);
    return vec4<f32>(input.color.rgb, alpha);
}
